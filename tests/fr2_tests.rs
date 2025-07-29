// Tests for FR2: Member Visibility Ordering Requirements

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

// FR2.1: Export Detection
// The system shall identify exported versus non-exported members in a file.

#[test]
fn test_fr2_1_detect_exported_functions() {
    let input = r#"
function privateFunc() {}
export function publicFunc() {}
const privateArrow = () => {};
export const publicArrow = () => {};
"#;
    
    let result = format_code(input);
    
    // Exported functions should be prioritized
    let public_func_pos = result.find("export function publicFunc").unwrap();
    let public_arrow_pos = result.find("export const publicArrow").unwrap();
    let private_func_pos = result.find("function privateFunc").unwrap();
    let private_arrow_pos = result.find("const privateArrow").unwrap();
    
    assert!(public_func_pos < private_func_pos);
    assert!(public_arrow_pos < private_arrow_pos);
}

#[test]
fn test_fr2_1_detect_exported_classes() {
    let input = r#"
class PrivateClass {}
export class PublicClass {}
"#;
    
    let result = format_code(input);
    
    let public_pos = result.find("export class PublicClass").unwrap();
    let private_pos = result.find("class PrivateClass").unwrap();
    
    assert!(public_pos < private_pos);
}

#[test]
fn test_fr2_1_detect_exported_interfaces() {
    let input = r#"
interface PrivateInterface {}
export interface PublicInterface {}
"#;
    
    let result = format_code(input);
    
    let public_pos = result.find("export interface PublicInterface").unwrap();
    let private_pos = result.find("interface PrivateInterface").unwrap();
    
    assert!(public_pos < private_pos);
}

#[test]
fn test_fr2_1_detect_exported_types() {
    let input = r#"
type PrivateType = string;
export type PublicType = string;
"#;
    
    let result = format_code(input);
    
    let public_pos = result.find("export type PublicType").unwrap();
    let private_pos = result.find("type PrivateType").unwrap();
    
    assert!(public_pos < private_pos);
}

#[test]
fn test_fr2_1_detect_exported_enums() {
    let input = r#"
enum PrivateEnum { A, B }
export enum PublicEnum { A, B }
"#;
    
    let result = format_code(input);
    
    let public_pos = result.find("export enum PublicEnum").unwrap();
    let private_pos = result.find("enum PrivateEnum").unwrap();
    
    assert!(public_pos < private_pos);
}

#[test]
fn test_fr2_1_detect_named_exports() {
    let input = r#"
const a = 1;
const b = 2;
export { a, b };
"#;
    
    let result = format_code(input);
    
    // Named exports should be recognized
    assert!(result.contains("const a = 1"));
    assert!(result.contains("const b = 2"));
    assert!(result.contains("export { a, b }"));
}

// FR2.2: Export Prioritization
// The system shall move exported members toward the top of the file when possible.

#[test]
fn test_fr2_2_prioritize_exported_members() {
    let input = r#"
const privateVar = 1;
export const publicVar = 2;
function privateFunc() {}
export function publicFunc() {}
"#;
    
    let result = format_code(input);
    
    // All exports should come before non-exports
    let public_var_pos = result.find("export const publicVar").unwrap();
    let public_func_pos = result.find("export function publicFunc").unwrap();
    let private_var_pos = result.find("const privateVar").unwrap();
    let private_func_pos = result.find("function privateFunc").unwrap();
    
    assert!(public_var_pos < private_var_pos);
    assert!(public_func_pos < private_func_pos);
}

#[test]
fn test_fr2_2_mixed_export_types() {
    let input = r#"
interface PrivateInterface {}
export class PublicClass {}
type PrivateType = string;
export function publicFunc() {}
const privateConst = 1;
export interface PublicInterface {}
"#;
    
    let result = format_code(input);
    
    // All exports should be prioritized
    let exports = vec![
        result.find("export class PublicClass").unwrap(),
        result.find("export function publicFunc").unwrap(),
        result.find("export interface PublicInterface").unwrap(),
    ];
    
    let non_exports = vec![
        result.find("interface PrivateInterface").unwrap(),
        result.find("type PrivateType").unwrap(),
        result.find("const privateConst").unwrap(),
    ];
    
    // All exports should come before all non-exports
    for export_pos in &exports {
        for non_export_pos in &non_exports {
            assert!(export_pos < non_export_pos);
        }
    }
}

// FR2.3: Dependency Preservation
// The system shall never reorder members in a way that breaks code functionality.

#[test]
fn test_fr2_3_preserve_variable_dependencies() {
    let input = r#"
const baseValue = 10;
export const derivedValue = baseValue * 2;
"#;
    
    let result = format_code(input);
    
    // baseValue must come before derivedValue
    let base_pos = result.find("const baseValue").unwrap();
    let derived_pos = result.find("export const derivedValue").unwrap();
    
    assert!(base_pos < derived_pos);
}

#[test]
fn test_fr2_3_preserve_function_dependencies() {
    let input = r#"
function helper() { return 42; }
export function main() { return helper(); }
"#;
    
    let result = format_code(input);
    
    // helper must be defined before main uses it
    let helper_pos = result.find("function helper").unwrap();
    let main_pos = result.find("export function main").unwrap();
    
    assert!(helper_pos < main_pos);
}

#[test]
fn test_fr2_3_preserve_class_inheritance() {
    let input = r#"
class BaseClass {}
export class DerivedClass extends BaseClass {}
"#;
    
    let result = format_code(input);
    
    // BaseClass must come before DerivedClass
    let base_pos = result.find("class BaseClass").unwrap();
    let derived_pos = result.find("export class DerivedClass").unwrap();
    
    assert!(base_pos < derived_pos);
}

#[test]
fn test_fr2_3_preserve_type_dependencies() {
    let input = r#"
type BaseType = string;
export type ExtendedType = BaseType | number;
"#;
    
    let result = format_code(input);
    
    // BaseType must come before ExtendedType
    let base_pos = result.find("type BaseType").unwrap();
    let extended_pos = result.find("export type ExtendedType").unwrap();
    
    assert!(base_pos < extended_pos);
}

#[test]
fn test_fr2_3_complex_dependency_chain() {
    let input = r#"
const a = 1;
const b = a + 1;
export const c = b + 1;
const d = c + 1;
export const e = d + 1;
"#;
    
    let result = format_code(input);
    
    // Order must be preserved: a -> b -> c -> d -> e
    let a_pos = result.find("const a = 1").unwrap();
    let b_pos = result.find("const b = a + 1").unwrap();
    let c_pos = result.find("export const c = b + 1").unwrap();
    let d_pos = result.find("const d = c + 1").unwrap();
    let e_pos = result.find("export const e = d + 1").unwrap();
    
    assert!(a_pos < b_pos);
    assert!(b_pos < c_pos);
    assert!(c_pos < d_pos);
    assert!(d_pos < e_pos);
}

// FR2.4: Intelligent Grouping
// The system shall keep related members together when reordering.

#[test]
fn test_fr2_4_keep_class_methods_together() {
    let input = r#"
export class MyClass {
    method1() {}
    method2() {}
}

function unrelatedFunc() {}
"#;
    
    let result = format_code(input);
    
    // Class and its methods should stay together
    assert!(result.contains(r#"export class MyClass {
    method1() {}"#));
}

#[test]
fn test_fr2_4_keep_interface_implementations_together() {
    let input = r#"
export interface MyInterface {
    prop: string;
}

export const implementation: MyInterface = {
    prop: 'value'
};
"#;
    
    let result = format_code(input);
    
    // Interface and its implementation should be close
    let interface_pos = result.find("export interface MyInterface").unwrap();
    let impl_pos = result.find("export const implementation").unwrap();
    
    // They should be relatively close (no large gap)
    assert!((impl_pos as i32 - interface_pos as i32) < 200);
}

#[test]
fn test_fr2_4_keep_type_guards_together() {
    let input = r#"
export type User = { name: string };

export function isUser(obj: any): obj is User {
    return obj && typeof obj.name === 'string';
}
"#;
    
    let result = format_code(input);
    
    // Type and its guard should be close
    let type_pos = result.find("export type User").unwrap();
    let guard_pos = result.find("export function isUser").unwrap();
    
    // They should be relatively close
    assert!((guard_pos as i32 - type_pos as i32) < 200);
}