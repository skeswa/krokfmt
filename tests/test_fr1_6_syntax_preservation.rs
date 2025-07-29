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

#[test]
fn test_preserve_default_imports() {
    let input = r#"
import React from 'react';
import lodash from 'lodash';
"#;

    let result = format_code(input);
    assert!(result.contains("import React from 'react';"));
    assert!(result.contains("import lodash from 'lodash';"));
}

#[test]
fn test_preserve_named_imports() {
    let input = r#"
import { useState, useEffect } from 'react';
import { debounce, throttle } from 'lodash';
"#;

    let result = format_code(input);
    assert!(result.contains("import { useState, useEffect } from 'react';"));
    assert!(result.contains("import { debounce, throttle } from 'lodash';"));
}

#[test]
fn test_preserve_namespace_imports() {
    let input = r#"
import * as fs from 'fs';
import * as utils from './utils';
"#;

    let result = format_code(input);
    assert!(result.contains("import * as fs from 'fs';"));
    assert!(result.contains("import * as utils from './utils';"));
}

#[test]
fn test_preserve_side_effect_imports() {
    let input = r#"
import './polyfills';
import 'reflect-metadata';
"#;

    let result = format_code(input);
    assert!(result.contains("import './polyfills';"));
    assert!(result.contains("import 'reflect-metadata';"));
}

#[test]
fn test_preserve_type_imports() {
    let input = r#"
import type { User } from './types';
import type { Config } from '@config/types';
"#;

    let result = format_code(input);
    assert!(result.contains("import type { User } from './types';"));
    assert!(result.contains("import type { Config } from '@config/types';"));
}

#[test]
fn test_preserve_import_aliases() {
    let input = r#"
import { foo as bar } from './module';
import { Component as MyComponent } from '@ui/components';
"#;

    let result = format_code(input);
    assert!(result.contains("import { foo as bar } from './module';"));
    assert!(result.contains("import { Component as MyComponent } from '@ui/components';"));
}

#[test]
fn test_preserve_mixed_imports() {
    let input = r#"
import React, { useState, useEffect } from 'react';
import utils, { helper, type HelperType } from '../utils/helpers';
"#;

    let result = format_code(input);
    assert!(result.contains("import React, { useState, useEffect } from 'react';"));
    assert!(result.contains("import utils, { helper, type HelperType } from '../utils/helpers';"));
}

#[test]
fn test_preserve_import_assertions() {
    let input = r#"
import data from './data.json' assert { type: 'json' };
import styles from './styles.css' assert { type: 'css' };
"#;

    let result = format_code(input);
    // SWC converts 'assert' to 'with' syntax (both are valid)
    assert!(result.contains("import data from './data.json' with"));
    assert!(result.contains("type: 'json'"));
    assert!(result.contains("import styles from './styles.css' with"));
    assert!(result.contains("type: 'css'"));
}

#[test]
fn test_preserve_export_from() {
    let input = r#"
export { default as MyExport } from './my-export';
export * from './re-exports';
export { foo, bar } from './module';
"#;

    let result = format_code(input);
    assert!(result.contains("export { default as MyExport } from './my-export';"));
    assert!(result.contains("export * from './re-exports';"));
    assert!(result.contains("export { foo, bar } from './module';"));
}
