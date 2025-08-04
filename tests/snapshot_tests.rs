use insta::assert_snapshot;
use krokfmt::{parser::TypeScriptParser, two_phase_formatter::TwoPhaseFormatter};
use std::fs;

fn format_code(input: &str) -> String {
    let parser = TypeScriptParser::new();
    let source_map = parser.source_map.clone();
    let comments = parser.comments.clone();
    let filename = if input.contains("<") && input.contains(">") {
        "test.tsx"
    } else {
        "test.ts"
    };
    let module = parser.parse(input, filename).unwrap();
    let formatter = TwoPhaseFormatter::new(source_map, comments);
    formatter
        .format_with_source(module, input.to_string())
        .unwrap()
}

fn test_fixture(fixture_path: &str) {
    test_fixture_with_extension(fixture_path, "ts");
}

fn test_fixture_with_extension(fixture_path: &str, extension: &str) {
    let input_path = format!("tests/fixtures/{fixture_path}.input.{extension}");
    let input = fs::read_to_string(&input_path)
        .unwrap_or_else(|_| panic!("Failed to read fixture: {input_path}"));

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
fn test_fr1_3_case_insensitive_sorting() {
    test_fixture("fr1/1_3_case_insensitive_sorting");
}

#[test]
fn test_fr1_4_positioning() {
    test_fixture_with_extension("fr1/1_4_positioning", "tsx");
}

#[test]
fn test_fr1_5_group_separation() {
    test_fixture("fr1/1_5_group_separation");
}

#[test]
fn test_fr1_6_syntax_preservation() {
    test_fixture("fr1/1_6_syntax_preservation");
}

#[test]
fn test_fr1_7_re_export_organization() {
    test_fixture("fr1/1_7_re_export_organization");
}

#[test]
fn test_fr1_7_re_export_sorting() {
    test_fixture("fr1/1_7_re_export_sorting");
}

#[test]
fn test_fr1_7_re_export_with_comments() {
    test_fixture("fr1/1_7_re_export_with_comments");
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
fn test_fr2_3_circular_reference() {
    test_fixture("fr2/2_3_circular_reference");
}

#[test]
fn test_fr2_3_complex_chains() {
    test_fixture("fr2/2_3_complex_chains");
}

#[test]
fn test_fr2_3_interleaved_dependencies() {
    test_fixture("fr2/2_3_interleaved_dependencies");
}

#[test]
fn test_fr2_3_class_dependencies() {
    test_fixture("fr2/2_3_class_dependencies");
}

#[test]
fn test_fr2_3_namespace_dependencies() {
    test_fixture("fr2/2_3_namespace_dependencies");
}

#[test]
fn test_fr2_3_destructuring_dependencies() {
    test_fixture("fr2/2_3_destructuring_dependencies");
}

#[test]
fn test_fr2_3_computed_properties() {
    test_fixture("fr2/2_3_computed_properties");
}

#[test]
fn test_fr2_3_hoisting_challenges() {
    test_fixture("fr2/2_3_hoisting_challenges");
}

#[test]
#[ignore = "Known issue: Comments separated by blank lines from type aliases may not be preserved correctly"]
fn test_fr2_3_forward_references() {
    test_fixture("fr2/2_3_forward_references");
}

#[test]
fn test_fr2_4_visibility_grouping() {
    test_fixture("fr2/2_4_visibility_grouping");
}

#[test]
fn test_fr2_4_visibility_with_dependencies() {
    test_fixture("fr2/2_4_visibility_with_dependencies");
}

#[test]
fn test_fr2_4_visibility_alphabetization() {
    test_fixture("fr2/2_4_visibility_alphabetization");
}

#[test]
fn test_fr2_4_case_insensitive_visibility() {
    test_fixture("fr2/2_4_case_insensitive_visibility");
}

#[test]
fn test_fr2_1_export_detection_edge_cases() {
    test_fixture("fr2/2_1_export_detection_edge_cases");
}

#[test]
fn test_fr2_2_export_prioritization_complex() {
    test_fixture("fr2/2_2_export_prioritization_complex");
}

#[test]
fn test_fr2_4_visibility_mixed_declarations() {
    test_fixture("fr2/2_4_visibility_mixed_declarations");
}

#[test]
fn test_fr2_comprehensive_integration() {
    test_fixture("fr2/2_comprehensive_integration");
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
fn test_fr3_2_case_insensitive_object_props() {
    test_fixture("fr3/3_2_case_insensitive_object_props");
}

#[test]
fn test_fr3_3_class_members() {
    test_fixture("fr3/3_3_class_members");
}

#[test]
fn test_fr3_3_visibility_groups() {
    test_fixture("fr3/3_3_visibility_groups");
}

#[test]
fn test_fr3_3_private_syntax() {
    test_fixture("fr3/3_3_private_syntax");
}

#[test]
fn test_fr3_3_mixed_visibility() {
    test_fixture("fr3/3_3_mixed_visibility");
}

#[test]
fn test_fr3_3_typescript_private() {
    test_fixture("fr3/3_3_typescript_private");
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

// FR6: Comment Handling Tests

#[test]
fn test_fr6_1_import_comments() {
    test_fixture("fr6/6_1_import_comments");
}

#[test]
fn test_fr6_2_export_comments() {
    test_fixture("fr6/6_2_export_comments");
}

#[test]
fn test_fr6_3_class_member_comments() {
    test_fixture("fr6/6_3_class_member_comments");
}

#[test]
fn test_fr6_4_object_property_comments() {
    test_fixture("fr6/6_4_object_property_comments");
}

#[test]
#[ignore = "Known issue: JSX comments ({/* */}) are not yet supported by the comment extraction system"]
fn test_fr6_5_jsx_comments() {
    test_fixture("fr6/6_5_jsx_comments");
}

#[test]
fn test_fr6_6_complex_comments() {
    test_fixture("fr6/6_6_complex_comments");
}

// FR7: Visual Separation Tests

#[test]
fn test_fr7_1_module_separation() {
    test_fixture("fr7/7_1_module_separation");
}

#[test]
fn test_fr7_3_class_member_separation() {
    test_fixture("fr7/7_3_class_member_separation");
}
