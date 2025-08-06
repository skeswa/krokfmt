# Configuration

krokfmt is a **zero-configuration** formatter. This is by design.

## No Configuration Files

krokfmt does not support, read, or require any configuration files:

- No `.krokfmtrc`
- No `krokfmt.config.js`
- No `package.json` settings
- No CLI options for style

## Why Zero Configuration?

### Consistency

Every project formatted with krokfmt looks the same. This means:
- Developers can move between projects seamlessly
- No learning curve for new team members
- No style guide documentation needed

### Simplicity

Zero configuration means:
- Nothing to set up
- Nothing to maintain
- Nothing to debate
- Nothing to break

### Focus

Without configuration options, teams can focus on:
- Writing code
- Solving problems
- Shipping features
- Not arguing about semicolons

## The krokfmt Style

While not configurable, here's what krokfmt does:

### Indentation
- **2 spaces** for all indentation
- No tabs

### Line Length
- **100 characters** maximum
- Intelligent line breaking

### Semicolons
- **Always** uses semicolons

### Quotes
- **Single quotes** for strings
- **Double quotes** for JSX attributes

### Trailing Commas
- **Always** in multi-line arrays and objects
- **Never** in single-line structures

### Brackets
- **Same line** for opening braces
- **New line** for else statements

### Spacing
- **Spaces** around operators
- **No space** before function call parentheses
- **Space** after keywords (if, for, while)

## File Selection

While you can't configure the style, you can control which files are formatted:

### Supported File Types

krokfmt automatically detects and formats:
- `.ts` - TypeScript
- `.tsx` - TypeScript with JSX
- `.mts` - ES Module TypeScript
- `.cts` - CommonJS TypeScript

### Ignoring Files

krokfmt respects `.gitignore` by default. To ignore additional files, use your shell:

```bash
# Format all except tests
krokfmt $(find src -name "*.ts" -not -path "*/test/*")

# Format only specific directories
krokfmt src/components src/utils
```

## Environment Variables

While not configuration per se, these control krokfmt's behavior:

### `KROKFMT_LOG`

Control logging verbosity:

```bash
KROKFMT_LOG=debug krokfmt .
```

Levels: `error`, `warn`, `info`, `debug`, `trace`

### `NO_COLOR`

Disable colored output:

```bash
NO_COLOR=1 krokfmt .
```

## Comparison with Other Tools

### Prettier

Prettier has 20+ configuration options. krokfmt has 0.

```json
// .prettierrc (not needed with krokfmt!)
{
  "semi": true,
  "singleQuote": true,
  "tabWidth": 2,
  "trailingComma": "all",
  "printWidth": 100,
  "arrowParens": "always",
  "endOfLine": "lf"
  // ... and many more
}
```

### ESLint

ESLint has hundreds of rules. krokfmt has one way.

```json
// .eslintrc (not needed with krokfmt!)
{
  "rules": {
    "indent": ["error", 2],
    "quotes": ["error", "single"],
    "semi": ["error", "always"]
    // ... hundreds more
  }
}
```

## FAQ

### Can I change the indentation?

No. It's always 2 spaces.

### Can I disable semicolons?

No. Semicolons are always added.

### Can I use double quotes?

No. Single quotes for strings (except JSX attributes).

### Can I change the line length?

No. It's set at 100 characters.

### What if I need different formatting?

krokfmt might not be the right tool for you. Consider:
- Prettier (highly configurable)
- ESLint (rule-based)
- dprint (some configuration)

### Will configuration ever be added?

No. Zero configuration is a core principle of krokfmt.

## Philosophy

> "The best configuration is no configuration."

krokfmt believes that:

1. **Debates about style are wasteful**
2. **Consistency matters more than preference**
3. **Decisions should be made once, by the tool**
4. **Simplicity is a feature**

## Benefits

By having no configuration, krokfmt provides:

- **Zero setup time**
- **No onboarding friction**
- **Consistent codebases**
- **No maintenance burden**
- **Clear expectations**
- **Fast decision making**

## Conclusion

If you're looking for a formatter you can configure to match your preferences, krokfmt is not for you.

If you want to stop thinking about formatting and just write code, welcome to krokfmt.