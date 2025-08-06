# What is krokfmt?

krokfmt is a highly opinionated, zero-configuration TypeScript code formatter written in Rust. It takes a different approach from other formatters by eliminating all configuration options in favor of a single, consistent style.

## Philosophy

### Zero Configuration

Unlike other formatters that offer hundreds of configuration options, krokfmt has **none**. This eliminates:

- Team debates about code style
- Configuration file maintenance
- Inconsistencies across projects
- Decision fatigue

### Opinionated by Design

krokfmt makes formatting decisions for you based on best practices and readability research:

- Consistent indentation (2 spaces)
- Alphabetically sorted properties
- Organized imports
- Smart line breaking
- Intelligent comment preservation

### Performance First

Written in Rust, krokfmt is designed for speed:

- Parallel file processing
- Efficient AST manipulation
- Minimal memory footprint
- Fast startup time

## How It Works

krokfmt uses a two-phase approach:

1. **Organization Phase**: Analyzes and reorganizes code structure
   - Sorts imports into categories
   - Orders class members by visibility
   - Alphabetizes object properties
   - Preserves semantic meaning

2. **Formatting Phase**: Applies consistent styling
   - Fixes indentation
   - Adds/removes whitespace
   - Ensures consistent punctuation
   - Wraps long lines

## Comparison with Other Formatters

| Feature | krokfmt | Prettier | ESLint | dprint |
|---------|---------|----------|--------|--------|
| Zero config | ✅ | ❌ | ❌ | ❌ |
| Import organization | ✅ | ❌ | Plugin | ✅ |
| Property sorting | ✅ | ❌ | Plugin | ❌ |
| TypeScript native | ✅ | ✅ | ✅ | ✅ |
| Speed | Fast | Moderate | Slow | Fast |
| Written in | Rust | JavaScript | JavaScript | Rust |

## Use Cases

krokfmt is perfect for:

- **Teams** tired of style debates
- **Projects** needing consistent formatting
- **CI/CD pipelines** requiring fast checks
- **Developers** who want to focus on code, not formatting
- **AI-generated code** that needs cleanup

## Design Principles

1. **Consistency over configurability**
2. **Performance without compromise**
3. **Preservation of developer intent**
4. **Semantic awareness**
5. **Zero learning curve**

## Who Should Use krokfmt?

- Teams that want to eliminate style discussions
- Projects with mixed skill levels
- Developers using AI code generation
- Anyone who values consistency over customization

## Who Shouldn't Use krokfmt?

- Teams with established, different style guides
- Projects requiring specific formatting rules
- Developers who need fine-grained control

## Next Steps

Ready to get started? Check out the [Getting Started](/guide/getting-started) guide to install and use krokfmt in your project.