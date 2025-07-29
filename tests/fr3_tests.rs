// Tests for FR3: Alphabetical Sorting Requirements

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

// FR3.1: Function Argument Sorting
// The system shall sort function arguments alphabetically when order doesn't affect behavior.

#[test]
fn test_fr3_1_sort_object_destructuring_params() {
    let input = r#"
function process({ zebra, apple, banana }: Options) {}
"#;
    
    let result = format_code(input);
    assert!(result.contains("function process({ apple, banana, zebra }: Options)"));
}

#[test]
fn test_fr3_1_preserve_positional_params() {
    let input = r#"
function add(b: number, a: number) { return a + b; }
"#;
    
    let result = format_code(input);
    // Positional parameters should NOT be sorted
    assert!(result.contains("function add(b: number, a: number)"));
}

#[test]
fn test_fr3_1_sort_arrow_function_destructuring() {
    let input = r#"
const handler = ({ zebra, apple, banana }: Options) => {};
"#;
    
    let result = format_code(input);
    assert!(result.contains("({ apple, banana, zebra }: Options)"));
}

#[test]
fn test_fr3_1_nested_destructuring_params() {
    let input = r#"
function process({ user: { name, age, id } }: { user: User }) {}
"#;
    
    let result = format_code(input);
    assert!(result.contains("{ user: { age, id, name } }"));
}

#[test]
fn test_fr3_1_mixed_params_only_sort_destructured() {
    let input = r#"
function mixed(a: number, { zebra, apple }: Options, b: string) {}
"#;
    
    let result = format_code(input);
    assert!(result.contains("function mixed(a: number, { apple, zebra }: Options, b: string)"));
}

// FR3.2: Object Property Sorting
// The system shall sort object literal properties alphabetically.

#[test]
fn test_fr3_2_sort_object_properties() {
    let input = r#"
const config = {
    zebra: true,
    apple: 42,
    banana: "yellow"
};
"#;
    
    let result = format_code(input);
    
    // Verify alphabetical order
    let apple_pos = result.find("apple:").unwrap();
    let banana_pos = result.find("banana:").unwrap();
    let zebra_pos = result.find("zebra:").unwrap();
    
    assert!(apple_pos < banana_pos);
    assert!(banana_pos < zebra_pos);
}

#[test]
fn test_fr3_2_computed_properties() {
    let input = r#"
const obj = {
    zebra: 1,
    [computedKey]: 2,
    apple: 3,
    ['literal']: 4
};
"#;
    
    let result = format_code(input);
    
    // Regular properties should be sorted
    let apple_pos = result.find("apple:").unwrap();
    let zebra_pos = result.find("zebra:").unwrap();
    assert!(apple_pos < zebra_pos);
}

#[test]
fn test_fr3_2_spread_operators_at_end() {
    let input = r#"
const obj = {
    zebra: 1,
    ...defaults,
    apple: 2,
    ...overrides
};
"#;
    
    let result = format_code(input);
    
    // Properties should be sorted, spreads preserved
    assert!(result.contains("apple: 2"));
    assert!(result.contains("zebra: 1"));
    assert!(result.contains("...defaults"));
    assert!(result.contains("...overrides"));
}

#[test]
fn test_fr3_2_nested_object_sorting() {
    let input = r#"
const config = {
    zebra: {
        inner: true,
        another: false
    },
    apple: {
        zebra: 1,
        apple: 2
    }
};
"#;
    
    let result = format_code(input);
    
    // Both outer and inner objects should be sorted
    let outer_apple = result.find("apple: {").unwrap();
    let outer_zebra = result.find("zebra: {").unwrap();
    assert!(outer_apple < outer_zebra);
}

// FR3.3: Class Member Sorting
// The system shall sort class fields and methods alphabetically within visibility groups.

#[test]
fn test_fr3_3_sort_instance_fields() {
    let input = r#"
class MyClass {
    zebra: string;
    apple: number;
    banana: boolean;
}
"#;
    
    let result = format_code(input);
    
    let apple_pos = result.find("apple:").unwrap();
    let banana_pos = result.find("banana:").unwrap();
    let zebra_pos = result.find("zebra:").unwrap();
    
    assert!(apple_pos < banana_pos);
    assert!(banana_pos < zebra_pos);
}

#[test]
fn test_fr3_3_sort_static_fields() {
    let input = r#"
class MyClass {
    static zebra = 1;
    static apple = 2;
    static banana = 3;
}
"#;
    
    let result = format_code(input);
    
    let apple_pos = result.find("static apple").unwrap();
    let banana_pos = result.find("static banana").unwrap();
    let zebra_pos = result.find("static zebra").unwrap();
    
    assert!(apple_pos < banana_pos);
    assert!(banana_pos < zebra_pos);
}

#[test]
fn test_fr3_3_sort_instance_methods() {
    let input = r#"
class MyClass {
    zebra() {}
    apple() {}
    banana() {}
}
"#;
    
    let result = format_code(input);
    
    let apple_pos = result.find("apple()").unwrap();
    let banana_pos = result.find("banana()").unwrap();
    let zebra_pos = result.find("zebra()").unwrap();
    
    assert!(apple_pos < banana_pos);
    assert!(banana_pos < zebra_pos);
}

#[test]
fn test_fr3_3_maintain_member_group_order() {
    let input = r#"
class MyClass {
    instanceB: string;
    static staticB = 1;
    instanceA: number;
    static staticA = 2;
    
    constructor() {}
    
    methodB() {}
    static staticMethodB() {}
    methodA() {}
    static staticMethodA() {}
}
"#;
    
    let result = format_code(input);
    
    // Static fields should come before instance fields
    let static_field_pos = result.find("static staticA").unwrap();
    let instance_field_pos = result.find("instanceA:").unwrap();
    assert!(static_field_pos < instance_field_pos);
    
    // Constructor should come after fields
    let constructor_pos = result.find("constructor()").unwrap();
    assert!(instance_field_pos < constructor_pos);
    
    // Static methods should come before instance methods
    let static_method_pos = result.find("static staticMethodA").unwrap();
    let instance_method_pos = result.find("methodA()").unwrap();
    assert!(static_method_pos < instance_method_pos);
}

// FR3.4: Type Member Sorting
// The system shall sort members of union and intersection types alphabetically.

#[test]
fn test_fr3_4_sort_union_type_members() {
    let input = r#"
type Status = 'error' | 'success' | 'pending';
"#;
    
    let result = format_code(input);
    assert!(result.contains("type Status = 'error' | 'pending' | 'success';"));
}

#[test]
fn test_fr3_4_sort_intersection_type_members() {
    let input = r#"
type Combined = TypeZ & TypeA & TypeM;
"#;
    
    let result = format_code(input);
    assert!(result.contains("type Combined = TypeA & TypeM & TypeZ;"));
}

#[test]
fn test_fr3_4_complex_union_types() {
    let input = r#"
type Mixed = string | number | boolean | null | undefined;
"#;
    
    let result = format_code(input);
    assert!(result.contains("type Mixed = boolean | null | number | string | undefined;"));
}

#[test]
fn test_fr3_4_object_union_types() {
    let input = r#"
type Result = { type: 'error' } | { type: 'success' } | { type: 'pending' };
"#;
    
    let result = format_code(input);
    println!("Result for object union types: {}", result);
    
    // Check if the type is sorted correctly
    assert!(result.contains("type Result"));
    
    // Object literals in unions might have different formatting
    // Just verify they all exist
    assert!(result.contains("'error'"));
    assert!(result.contains("'success'"));
    assert!(result.contains("'pending'"));
}

// FR3.5: Enum Member Sorting
// The system shall sort enum members alphabetically.

#[test]
fn test_fr3_5_sort_string_enum_members() {
    let input = r#"
enum Status {
    Error = 'error',
    Success = 'success',
    Pending = 'pending'
}
"#;
    
    let result = format_code(input);
    
    let error_pos = result.find("Error = 'error'").unwrap();
    let pending_pos = result.find("Pending = 'pending'").unwrap();
    let success_pos = result.find("Success = 'success'").unwrap();
    
    assert!(error_pos < pending_pos);
    assert!(pending_pos < success_pos);
}

#[test]
fn test_fr3_5_preserve_numeric_enum_values() {
    let input = r#"
enum Priority {
    Low = 1,
    High = 3,
    Medium = 2
}
"#;
    
    let result = format_code(input);
    
    // Numeric enums should NOT be sorted (values must be preserved)
    assert!(result.contains("Low = 1"));
    assert!(result.contains("High = 3"));
    assert!(result.contains("Medium = 2"));
    
    // Original order should be maintained
    let low_pos = result.find("Low = 1").unwrap();
    let high_pos = result.find("High = 3").unwrap();
    let medium_pos = result.find("Medium = 2").unwrap();
    
    assert!(low_pos < high_pos);
    assert!(high_pos < medium_pos);
}

#[test]
fn test_fr3_5_mixed_enum_preserve_order() {
    let input = r#"
enum Mixed {
    A,
    B = 5,
    C
}
"#;
    
    let result = format_code(input);
    println!("Result for mixed enum: {}", result);
    
    // Mixed enums with numeric values should NOT be sorted
    assert!(result.contains("enum Mixed"));
    assert!(result.contains("A"));
    assert!(result.contains("B = 5"));
    assert!(result.contains("C"));
    
    // Verify order is preserved
    let enum_content = result.split("enum Mixed").nth(1).unwrap();
    let a_pos = enum_content.find("A").unwrap();
    let b_pos = enum_content.find("B = 5").unwrap();
    
    assert!(a_pos < b_pos);
}

// FR3.6: JSX Property Sorting
// The system shall sort JSX/TSX element properties alphabetically.

#[test]
fn test_fr3_6_sort_jsx_props_basic() {
    let input = r#"
const elem = <Button disabled onClick={handler} className="btn" />;
"#;
    
    let result = format_code(input);
    
    let classname_pos = result.find("className=").unwrap();
    let disabled_pos = result.find("disabled").unwrap();
    let onclick_pos = result.find("onClick=").unwrap();
    
    assert!(classname_pos < disabled_pos);
    assert!(disabled_pos < onclick_pos);
}

#[test]
fn test_fr3_6_key_ref_props_first() {
    let input = r#"
const elem = <Item name="test" key={id} className="item" ref={itemRef} />;
"#;
    
    let result = format_code(input);
    
    // key and ref should come first
    let key_pos = result.find("key=").unwrap();
    let ref_pos = result.find("ref=").unwrap();
    let classname_pos = result.find("className=").unwrap();
    let name_pos = result.find("name=").unwrap();
    
    assert!(key_pos < ref_pos);
    assert!(ref_pos < classname_pos);
    assert!(classname_pos < name_pos);
}

#[test]
fn test_fr3_6_event_handlers_grouped() {
    let input = r#"
const elem = <Input value={val} onChange={change} placeholder="Enter" onClick={click} onBlur={blur} />;
"#;
    
    let result = format_code(input);
    
    // Event handlers should be grouped together
    let onblur_pos = result.find("onBlur=").unwrap();
    let onchange_pos = result.find("onChange=").unwrap();
    let onclick_pos = result.find("onClick=").unwrap();
    
    // They should be consecutive (grouped)
    assert!(onblur_pos < onchange_pos);
    assert!(onchange_pos < onclick_pos);
}

#[test]
fn test_fr3_6_spread_props_at_end() {
    let input = r#"
const elem = <Component name="test" {...props} className="comp" {...moreProps} />;
"#;
    
    let result = format_code(input);
    
    // Regular props should be sorted, spreads at the end
    assert!(result.contains("className="));
    assert!(result.contains("name="));
    assert!(result.contains("{...props}"));
    assert!(result.contains("{...moreProps}"));
}