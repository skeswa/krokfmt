use insta::assert_snapshot;
use krokfmt::{codegen::CodeGenerator, formatter::KrokFormatter, parser::TypeScriptParser};
use std::fs;

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

fn test_fixture(fixture_path: &str) {
    let input_path = format!("tests/fixtures/{}.input.ts", fixture_path);
    let input = fs::read_to_string(&input_path)
        .unwrap_or_else(|_| panic!("Failed to read fixture: {}", input_path));
    
    let output = format_code(&input);
    
    // Use the fixture path as the snapshot name
    assert_snapshot!(fixture_path, output);
}

// FR1: Import/Export Organization Tests

#[test]
fn test_fr1_1_default_imports() {
    test_fixture("fr1/1_1_default_imports");
}

#[test]
fn test_fr1_1_named_imports() {
    test_fixture("fr1/1_1_named_imports");
}

#[test]
fn test_fr1_1_namespace_imports() {
    test_fixture("fr1/1_1_namespace_imports");
}

#[test]
fn test_fr1_1_side_effect_imports() {
    test_fixture("fr1/1_1_side_effect_imports");
}

#[test]
fn test_fr1_1_type_imports() {
    test_fixture("fr1/1_1_type_imports");
}

#[test]
fn test_fr1_1_mixed_imports() {
    test_fixture("fr1/1_1_mixed_imports");
}

#[test]
fn test_fr1_1_import_aliases() {
    test_fixture("fr1/1_1_import_aliases");
}

#[test]
fn test_fr1_2_categorization() {
    test_fixture("fr1/1_2_categorization");
}

#[test]
fn test_fr1_3_sorting() {
    test_fixture("fr1/1_3_sorting");
}

#[test]
fn test_fr1_4_positioning() {
    test_fixture("fr1/1_4_positioning");
}

#[test]
fn test_fr1_5_group_separation() {
    test_fixture("fr1/1_5_group_separation");
}

#[test]
fn test_fr1_6_syntax_preservation() {
    test_fixture("fr1/1_6_syntax_preservation");
}

// FR2: Member Visibility Ordering Tests

#[test]
fn test_fr2_1_export_detection() {
    test_fixture("fr2/2_1_export_detection");
}

#[test]
fn test_fr2_2_export_prioritization() {
    test_fixture("fr2/2_2_export_prioritization");
}

#[test]
fn test_fr2_3_dependency_preservation() {
    test_fixture("fr2/2_3_dependency_preservation");
}

#[test]
fn test_fr2_4_intelligent_grouping() {
    test_fixture("fr2/2_4_intelligent_grouping");
}

// FR3: Alphabetical Sorting Tests

#[test]
fn test_fr3_1_function_arguments() {
    test_fixture("fr3/3_1_function_arguments");
}

#[test]
fn test_fr3_2_object_properties() {
    test_fixture("fr3/3_2_object_properties");
}

#[test]
fn test_fr3_3_class_members() {
    test_fixture("fr3/3_3_class_members");
}

#[test]
fn test_fr3_4_type_members() {
    test_fixture("fr3/3_4_type_members");
}

#[test]
fn test_fr3_5_enum_members() {
    test_fixture("fr3/3_5_enum_members");
}

#[test]
fn test_fr3_6_jsx_properties() {
    test_fixture("fr3/3_6_jsx_properties");
}