use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use krokfmt::{codegen::CodeGenerator, organizer::KrokOrganizer, parser::TypeScriptParser};
use std::fs;
use std::path::Path;

fn organize_code(input: &str) -> String {
    let parser = TypeScriptParser::new();
    let source_map = parser.source_map.clone();
    let comments = parser.comments.clone();
    let filename = if input.contains("<") && input.contains(">") {
        "test.tsx"
    } else {
        "test.ts"
    };
    let module = parser.parse(input, filename).unwrap();
    let organizer = KrokOrganizer::new();
    let organized_module = organizer.organize(module).unwrap();
    let generator = CodeGenerator::with_comments(source_map, comments);
    generator.generate(&organized_module).unwrap()
}

fn load_fixture(fixture_path: &str) -> String {
    let path = Path::new("tests/fixtures").join(fixture_path);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read fixture: {path:?}"))
}

fn bench_real_fixtures(c: &mut Criterion) {
    let fixtures = vec![
        ("small_imports", "fr1/1_1_mixed_imports.input.ts"),
        ("categorized_imports", "fr1/1_2_categorization.input.ts"),
        ("class_members", "fr3/3_3_class_members.input.ts"),
        ("complex_types", "fr3/3_4_type_members.input.ts"),
        ("jsx_props", "fr3/3_6_jsx_properties.input.ts"),
    ];

    let mut group = c.benchmark_group("real_world_formatting");

    for (name, fixture_path) in fixtures {
        let input = load_fixture(fixture_path);
        let size = input.len() as u64;

        group.throughput(Throughput::Bytes(size));
        group.bench_with_input(BenchmarkId::new("format", name), &input, |b, input| {
            b.iter(|| organize_code(black_box(input)))
        });
    }

    group.finish();
}

fn bench_parsing_only(c: &mut Criterion) {
    let input = load_fixture("fr1/1_1_mixed_imports.input.ts");

    c.bench_function("parse_only", |b| {
        b.iter(|| {
            let parser = TypeScriptParser::new();
            parser.parse(black_box(&input), "test.ts").unwrap()
        })
    });
}

fn bench_formatting_only(c: &mut Criterion) {
    let input = load_fixture("fr1/1_1_mixed_imports.input.ts");
    let parser = TypeScriptParser::new();
    let module = parser.parse(&input, "test.ts").unwrap();

    c.bench_function("organize_only", |b| {
        b.iter(|| {
            let organizer = KrokOrganizer::new();
            organizer.organize(black_box(module.clone())).unwrap()
        })
    });
}

fn bench_codegen_only(c: &mut Criterion) {
    let input = load_fixture("fr1/1_1_mixed_imports.input.ts");
    let parser = TypeScriptParser::new();
    let source_map = parser.source_map.clone();
    let module = parser.parse(&input, "test.ts").unwrap();
    let organized_module = KrokOrganizer::new().organize(module).unwrap();

    c.bench_function("codegen_only", |b| {
        b.iter(|| {
            let generator = CodeGenerator::new(source_map.clone());
            generator.generate(black_box(&organized_module)).unwrap()
        })
    });
}

criterion_group!(
    benches,
    bench_real_fixtures,
    bench_parsing_only,
    bench_formatting_only,
    bench_codegen_only
);
criterion_main!(benches);
