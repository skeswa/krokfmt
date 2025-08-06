# API Reference

## WebAssembly API (Recommended)

krokfmt runs directly in the browser via WebAssembly - no server required!

### Browser Integration

```html
<script type="module">
  import init, { format_typescript, init_panic_hook } from '/wasm/krokfmt_playground.js';
  
  // Initialize WASM module
  await init('/wasm/krokfmt_playground_bg.wasm');
  init_panic_hook(); // Better error messages
  
  // Format code
  const code = "const x={a:1,b:2}";
  const resultJson = format_typescript(code);
  const result = JSON.parse(resultJson);
  
  if (result.success) {
    console.log(result.formatted); // "const x = { a: 1, b: 2 };"
  } else {
    console.error(result.error);
  }
</script>
```

### NPM Package (Coming Soon)

```javascript
import { format } from 'krokfmt-wasm';

const formatted = await format("const x={a:1}");
```

### Benefits of WASM

- ⚡ **No network latency** - Runs entirely in the browser
- 🔒 **Privacy** - Code never leaves your machine
- 📴 **Works offline** - No internet connection needed
- 🚀 **Fast** - Native performance via WebAssembly
- 🔧 **No server costs** - Static hosting only


## Rust API

For Rust projects, you can use krokfmt as a library.

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
krokfmt = "0.1"
```

### Usage

```rust
use krokfmt::format_typescript;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = "const x={a:1,b:2}";
    let formatted = format_typescript(code, "input.ts")?;
    println!("{}", formatted);
    Ok(())
}
```

## CLI API

The krokfmt command-line interface.

### Synopsis

```bash
krokfmt [OPTIONS] [FILES...]
```

### Options

- `--check` - Check if files are formatted (exit with error if not)
- `--stdout` - Print formatted output to stdout instead of writing files
- `--watch` - Watch files for changes and format automatically
- `--version` - Print version information
- `--help` - Print help information

### Exit Codes

- `0` - Success
- `1` - Formatting errors or files need formatting (in check mode)
- `2` - Invalid arguments or configuration

### Environment Variables

- `KROKFMT_LOG` - Set log level (error, warn, info, debug, trace)
- `NO_COLOR` - Disable colored output