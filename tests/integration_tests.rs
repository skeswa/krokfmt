use krokfmt::{codegen::CodeGenerator, formatter::KrokFormatter, parser::TypeScriptParser};

fn format_code(input: &str) -> String {
    let parser = TypeScriptParser::new();
    let source_map = parser.source_map.clone();
    // Parse as TSX if the input contains JSX
    let filename = if input.contains("<") && input.contains(">") {
        "test.tsx"
    } else {
        "test.ts"
    };
    let module = parser.parse(input, filename).unwrap();
    let formatted = KrokFormatter::new().format(module).unwrap();
    let generator = CodeGenerator::new(source_map);
    generator.generate(&formatted).unwrap()
}

#[test]
fn test_import_organization_complete() {
    let input = r#"
// Some comment
import { z } from './utils/validation';
import React, { useState } from 'react';
import { Button } from '@ui/components/Button';
import axios from 'axios';
import { User } from '../types/user';
import { api } from '@services/api';
import lodash from 'lodash';

const App = () => {
    return <div>Hello</div>;
};
"#;

    let result = format_code(input);

    // Verify imports are in correct order
    let lines: Vec<&str> = result.lines().collect();
    let import_lines: Vec<&str> = lines
        .iter()
        .filter(|line| line.starts_with("import"))
        .copied()
        .collect();

    assert_eq!(import_lines.len(), 7);
    // External imports first
    assert!(import_lines[0].contains("axios"));
    assert!(import_lines[1].contains("lodash"));
    assert!(import_lines[2].contains("React"));
    // Absolute imports
    assert!(import_lines[3].contains("@services/api"));
    assert!(import_lines[4].contains("@ui/components/Button"));
    // Relative imports
    assert!(import_lines[5].contains("../types/user"));
    assert!(import_lines[6].contains("./utils/validation"));

    // Verify the rest of the code is preserved
    assert!(result.contains("const App"));
    assert!(result.contains("return <div>Hello</div>"));
}

#[test]
fn test_object_property_sorting() {
    let input = r#"
const config = {
    zebra: true,
    apple: 42,
    banana: "yellow",
    cat: null,
};
"#;

    let result = format_code(input);
    assert!(result.contains(r#"apple: 42"#));
    assert!(result.contains(r#"banana: "yellow""#));
    assert!(result.contains(r#"cat: null"#));
    assert!(result.contains(r#"zebra: true"#));

    // Verify order
    let apple_pos = result.find("apple").unwrap();
    let banana_pos = result.find("banana").unwrap();
    let cat_pos = result.find("cat").unwrap();
    let zebra_pos = result.find("zebra").unwrap();

    assert!(apple_pos < banana_pos);
    assert!(banana_pos < cat_pos);
    assert!(cat_pos < zebra_pos);
}

#[test]
fn test_preserve_side_effect_imports() {
    let input = r#"
import './polyfills';
import 'reflect-metadata';
import React from 'react';
"#;

    let result = format_code(input);
    assert!(result.contains("import './polyfills'"));
    assert!(result.contains("import 'reflect-metadata'"));
    assert!(result.contains("import React from 'react'"));
}

#[test]
fn test_preserve_type_imports() {
    let input = r#"
import type { User } from './types';
import { api } from './api';
import type { Config } from '@types/config';
"#;

    let result = format_code(input);
    assert!(result.contains("import type { Config } from '@types/config'"));
    assert!(result.contains("import type { User } from './types'"));
    assert!(result.contains("import { api } from './api'"));
}

#[test]
fn test_complex_real_world_file() {
    let input = r#"
import { useEffect, useState } from 'react';
import axios from 'axios';
import { Button } from '@mui/material';
import { useAuth } from '../hooks/auth';
import './styles.css';
import type { User, Profile } from '@types/user';
import { validateEmail } from './utils/validation';
import { API_URL } from '@config/constants';

export const UserProfile: React.FC = () => {
    const [user, setUser] = useState<User | null>(null);
    const { token } = useAuth();
    
    const config = {
        headers: {
            'Authorization': `Bearer ${token}`,
            'Content-Type': 'application/json',
        },
        timeout: 5000,
        baseURL: API_URL,
    };
    
    useEffect(() => {
        fetchUser();
    }, []);
    
    const fetchUser = async () => {
        try {
            const response = await axios.get('/user', config);
            setUser(response.data);
        } catch (error) {
            console.error('Failed to fetch user:', error);
        }
    };
    
    return (
        <div>
            {user && <h1>{user.name}</h1>}
            <Button onClick={fetchUser}>Refresh</Button>
        </div>
    );
};
"#;

    let result = format_code(input);

    // Verify imports are properly organized
    assert!(result.contains("import axios from 'axios'"));
    assert!(result.contains("import { useEffect, useState } from 'react'"));
    assert!(result.contains("import './styles.css'"));

    // Verify object properties are sorted
    assert!(result.contains("'Authorization': `Bearer ${token}`"));
    assert!(result.contains("baseURL: API_URL"));
    assert!(result.contains("'Content-Type': 'application/json'"));
    assert!(result.contains("headers:"));
    assert!(result.contains("timeout: 5000"));

    // Verify alphabetical order of properties
    let result_lower = result.to_lowercase();
    let base_url_pos = result_lower.find("baseurl").unwrap();
    let _content_type_pos = result_lower.find("content-type").unwrap();
    let headers_pos = result_lower.find("headers:").unwrap();
    let timeout_pos = result_lower.find("timeout").unwrap();

    assert!(base_url_pos < headers_pos);
    assert!(headers_pos < timeout_pos);
}

#[test]
fn test_nested_object_sorting() {
    let input = r#"
const nested = {
    z: {
        inner: true,
        another: false,
    },
    a: {
        zebra: 1,
        apple: 2,
    },
};
"#;

    let result = format_code(input);

    // Outer object sorted
    let a_pos = result.find("a:").unwrap();
    let z_pos = result.find("z:").unwrap();
    assert!(a_pos < z_pos);

    // Inner objects sorted
    assert!(result.contains("apple: 2"));
    assert!(result.contains("zebra: 1"));
    assert!(result.contains("another: false"));
    assert!(result.contains("inner: true"));
}

#[test]
fn test_preserve_spread_operators() {
    let input = r#"
const obj = {
    ...defaults,
    zebra: 1,
    apple: 2,
    ...overrides,
};
"#;

    let result = format_code(input);
    assert!(result.contains("...defaults"));
    assert!(result.contains("...overrides"));
    assert!(result.contains("apple: 2"));
    assert!(result.contains("zebra: 1"));
}
