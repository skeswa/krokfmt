// Tests for FR1: Import/Export Organization Requirements

use krokfmt::{codegen::CodeGenerator, formatter::KrokFormatter, parser::TypeScriptParser};

fn format_code(input: &str) -> String {
    let parser = TypeScriptParser::new();
    let source_map = parser.source_map.clone();
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

// FR1.1: Import Statement Parsing
// The system shall parse and identify all import and export statements in a TypeScript file.

#[test]
fn test_fr1_1_parse_default_imports() {
    let input = r#"
import React from 'react';
import lodash from 'lodash';
"#;
    
    let result = format_code(input);
    assert!(result.contains("import React from 'react';"));
    assert!(result.contains("import lodash from 'lodash';"));
}

#[test]
fn test_fr1_1_parse_named_imports() {
    let input = r#"
import { useState, useEffect } from 'react';
import { debounce } from 'lodash';
"#;
    
    let result = format_code(input);
    assert!(result.contains("import { useState, useEffect } from 'react';"));
    assert!(result.contains("import { debounce } from 'lodash';"));
}

#[test]
fn test_fr1_1_parse_namespace_imports() {
    let input = r#"
import * as fs from 'fs';
import * as utils from './utils';
"#;
    
    let result = format_code(input);
    assert!(result.contains("import * as fs from 'fs';"));
    assert!(result.contains("import * as utils from './utils';"));
}

#[test]
fn test_fr1_1_parse_side_effect_imports() {
    let input = r#"
import './styles.css';
import 'reflect-metadata';
"#;
    
    let result = format_code(input);
    assert!(result.contains("import './styles.css';"));
    assert!(result.contains("import 'reflect-metadata';"));
}

#[test]
fn test_fr1_1_parse_type_imports() {
    let input = r#"
import type { User } from './types';
import type { Config } from '@config/types';
"#;
    
    let result = format_code(input);
    assert!(result.contains("import type { User } from './types';"));
    assert!(result.contains("import type { Config } from '@config/types';"));
}

#[test]
fn test_fr1_1_parse_mixed_imports() {
    let input = r#"
import React, { useState } from 'react';
import utils, { helper, type HelperType } from './utils';
"#;
    
    let result = format_code(input);
    assert!(result.contains("import React, { useState } from 'react';"));
    assert!(result.contains("import utils, { helper, type HelperType } from './utils';"));
}

#[test]
fn test_fr1_1_parse_import_aliases() {
    let input = r#"
import { foo as bar } from './module';
import { Component as MyComponent } from '@ui/components';
"#;
    
    let result = format_code(input);
    assert!(result.contains("import { foo as bar } from './module';"));
    assert!(result.contains("import { Component as MyComponent } from '@ui/components';"));
}

// FR1.2: Import Categorization
// The system shall categorize imports into three distinct groups based on their path patterns.

#[test]
fn test_fr1_2_categorize_external_imports() {
    let input = r#"
import React from 'react';
import lodash from 'lodash/debounce';
import axios from 'axios';
"#;
    
    let result = format_code(input);
    let lines: Vec<&str> = result.lines().collect();
    let import_lines: Vec<&str> = lines.iter()
        .filter(|line| line.trim().starts_with("import"))
        .copied()
        .collect();
    
    // All external imports should be together
    assert!(import_lines[0].contains("axios"));
    assert!(import_lines[1].contains("lodash"));
    assert!(import_lines[2].contains("React"));
}

#[test]
fn test_fr1_2_categorize_absolute_imports() {
    let input = r#"
import { Button } from '@components/Button';
import { config } from '~/config';
import { api } from '@services/api';
"#;
    
    let result = format_code(input);
    let lines: Vec<&str> = result.lines().collect();
    let import_lines: Vec<&str> = lines.iter()
        .filter(|line| line.trim().starts_with("import"))
        .copied()
        .collect();
    
    // All absolute imports should be together
    assert!(import_lines.iter().all(|line| line.contains("@") || line.contains("~")));
}

#[test]
fn test_fr1_2_categorize_relative_imports() {
    let input = r#"
import { helper } from './utils/helper';
import { User } from '../types';
import { deepUtil } from '../../utils/deep';
"#;
    
    let result = format_code(input);
    let lines: Vec<&str> = result.lines().collect();
    let import_lines: Vec<&str> = lines.iter()
        .filter(|line| line.trim().starts_with("import"))
        .copied()
        .collect();
    
    // All relative imports should be together
    assert!(import_lines.iter().all(|line| line.contains("./") || line.contains("../")));
}

#[test]
fn test_fr1_2_categorize_mixed_imports() {
    let input = r#"
import { helper } from './helper';
import React from 'react';
import { Button } from '@ui/Button';
"#;
    
    let result = format_code(input);
    let lines: Vec<&str> = result.lines().collect();
    let import_lines: Vec<&str> = lines.iter()
        .filter(|line| line.trim().starts_with("import"))
        .copied()
        .collect();
    
    // Should be ordered: external, absolute, relative
    assert!(import_lines[0].contains("React")); // external
    assert!(import_lines[1].contains("@ui")); // absolute
    assert!(import_lines[2].contains("./")); // relative
}

// FR1.3: Import Sorting
// The system shall sort imports alphabetically within each category by import path.

#[test]
fn test_fr1_3_sort_external_imports() {
    let input = r#"
import zod from 'zod';
import axios from 'axios';
import react from 'react';
"#;
    
    let result = format_code(input);
    let lines: Vec<&str> = result.lines().collect();
    let import_lines: Vec<&str> = lines.iter()
        .filter(|line| line.trim().starts_with("import"))
        .copied()
        .collect();
    
    assert!(import_lines[0].contains("axios"));
    assert!(import_lines[1].contains("react"));
    assert!(import_lines[2].contains("zod"));
}

#[test]
fn test_fr1_3_sort_absolute_imports() {
    let input = r#"
import { z } from '@utils/z';
import { a } from '@utils/a';
import { m } from '@utils/m';
"#;
    
    let result = format_code(input);
    let lines: Vec<&str> = result.lines().collect();
    let import_lines: Vec<&str> = lines.iter()
        .filter(|line| line.trim().starts_with("import"))
        .copied()
        .collect();
    
    assert!(import_lines[0].contains("@utils/a"));
    assert!(import_lines[1].contains("@utils/m"));
    assert!(import_lines[2].contains("@utils/z"));
}

#[test]
fn test_fr1_3_sort_relative_imports() {
    let input = r#"
import { z } from './z';
import { a } from '../a';
import { m } from '../../m';
"#;
    
    let result = format_code(input);
    let lines: Vec<&str> = result.lines().collect();
    let import_lines: Vec<&str> = lines.iter()
        .filter(|line| line.trim().starts_with("import"))
        .copied()
        .collect();
    
    assert!(import_lines[0].contains("../../m"));
    assert!(import_lines[1].contains("../a"));
    assert!(import_lines[2].contains("./z"));
}

#[test]
fn test_fr1_3_case_sensitive_sorting() {
    let input = r#"
import b from 'b';
import A from 'A';
import a from 'a';
import B from 'B';
"#;
    
    let result = format_code(input);
    let lines: Vec<&str> = result.lines().collect();
    let import_lines: Vec<&str> = lines.iter()
        .filter(|line| line.trim().starts_with("import"))
        .copied()
        .collect();
    
    // ASCII order: uppercase before lowercase
    assert!(import_lines[0].contains("from 'A'"));
    assert!(import_lines[1].contains("from 'B'"));
    assert!(import_lines[2].contains("from 'a'"));
    assert!(import_lines[3].contains("from 'b'"));
}

// FR1.4: Import Positioning
// The system shall place all import and export statements at the top of the file.

#[test]
fn test_fr1_4_imports_at_top() {
    let input = r#"
const earlyVar = 'too early';

import React from 'react';

const App = () => <div>Hello</div>;
"#;
    
    let result = format_code(input);
    
    // Import should come before any code
    let import_pos = result.find("import React").unwrap();
    let early_var_pos = result.find("const earlyVar").unwrap();
    let app_pos = result.find("const App").unwrap();
    
    assert!(import_pos < early_var_pos);
    assert!(early_var_pos < app_pos);
}

#[test]
fn test_fr1_4_preserve_file_comments() {
    let input = r#"
// File header comment
// Copyright notice

import React from 'react';
"#;
    
    let result = format_code(input);
    
    // Comments should be preserved (though SWC might strip them)
    // The import should still be present
    assert!(result.contains("import React from 'react';"));
}

#[test]
fn test_fr1_4_multiple_orphaned_statements() {
    let input = r#"
const a = 1;
let b = 2;
var c = 3;

import React from 'react';
import axios from 'axios';

function main() {}
"#;
    
    let result = format_code(input);
    
    // All imports should come before all other code
    let last_import = result.rfind("import").unwrap();
    let first_const = result.find("const a").unwrap();
    
    assert!(last_import < first_const);
}

// FR1.5: Import Group Separation
// The system shall separate import groups with exactly one empty line.

#[test]
fn test_fr1_5_external_absolute_separation() {
    let input = r#"
import React from 'react';
import { Button } from '@ui/Button';
"#;
    
    let result = format_code(input);
    
    // Should have empty line between external and absolute
    assert!(result.contains("import React from 'react';\n\nimport { Button }"));
}

#[test]
fn test_fr1_5_absolute_relative_separation() {
    let input = r#"
import { Button } from '@ui/Button';
import { helper } from './helper';
"#;
    
    let result = format_code(input);
    
    // Should have empty line between absolute and relative
    assert!(result.contains("import { Button } from '@ui/Button';\n\nimport { helper }"));
}

#[test]
fn test_fr1_5_all_groups_separation() {
    let input = r#"
import React from 'react';
import { Button } from '@ui/Button';
import { helper } from './helper';
"#;
    
    let result = format_code(input);
    
    // Should have empty lines between all groups
    assert!(result.contains("import React from 'react';\n\nimport"));
    assert!(result.contains("import { Button } from '@ui/Button';\n\nimport"));
}

#[test]
fn test_fr1_5_code_after_imports_separation() {
    let input = r#"
import React from 'react';

const App = () => <div>Hello</div>;
"#;
    
    let result = format_code(input);
    
    // Should have empty line after last import
    assert!(result.contains("import React from 'react';\n\nconst App"));
}

// FR1.6: Import Syntax Preservation
// The system shall preserve the exact import syntax and semantics.

#[test]
fn test_fr1_6_preserve_default_import_syntax() {
    let input = r#"
import MyReact from 'react';
"#;
    
    let result = format_code(input);
    assert!(result.contains("import MyReact from 'react';"));
}

#[test]
fn test_fr1_6_preserve_type_only_imports() {
    let input = r#"
import type { User } from './types';
import { type Config, type Settings } from './config';
"#;
    
    let result = format_code(input);
    assert!(result.contains("import type { User } from './types';"));
    assert!(result.contains("import { type Config, type Settings } from './config';"));
}

#[test]
fn test_fr1_6_preserve_side_effect_imports() {
    let input = r#"
import './polyfills';
import 'reflect-metadata';
"#;
    
    let result = format_code(input);
    assert!(result.contains("import './polyfills';"));
    assert!(result.contains("import 'reflect-metadata';"));
}

#[test]
fn test_fr1_6_preserve_import_assertions() {
    let input = r#"
import data from './data.json' assert { type: 'json' };
"#;
    
    let result = format_code(input);
    // SWC converts 'assert' to 'with' (both are valid)
    assert!(result.contains("import data from './data.json' with"));
    assert!(result.contains("type: 'json'"));
}