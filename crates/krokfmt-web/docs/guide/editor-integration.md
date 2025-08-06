# Editor Integration

Integrate krokfmt with your favorite editor for automatic formatting on save.

## Visual Studio Code

### Manual Setup

1. Install krokfmt globally or locally
2. Open VS Code settings (`Cmd+,` or `Ctrl+,`)
3. Search for "Format On Save" and enable it
4. Add to `settings.json`:

```json
{
  "editor.formatOnSave": true,
  "[typescript]": {
    "editor.defaultFormatter": "krokfmt.vscode-krokfmt"
  },
  "[typescriptreact]": {
    "editor.defaultFormatter": "krokfmt.vscode-krokfmt"
  }
}
```

### Using Run on Save Extension

Install the "Run on Save" extension and add to `settings.json`:

```json
{
  "runOnSave.commands": [
    {
      "match": "\\.(ts|tsx)$",
      "command": "krokfmt ${file}",
      "runIn": "backend"
    }
  ]
}
```

## Neovim / Vim

### Using null-ls (Neovim)

```lua
local null_ls = require("null-ls")

null_ls.setup({
  sources = {
    null_ls.builtins.formatting.krokfmt,
  },
})

-- Format on save
vim.cmd([[
  augroup FormatAutogroup
    autocmd!
    autocmd BufWritePre *.ts,*.tsx lua vim.lsp.buf.format()
  augroup END
]])
```

### Using ALE (Vim/Neovim)

```vim
" Add krokfmt as a fixer
let g:ale_fixers = {
\   'typescript': ['krokfmt'],
\   'typescriptreact': ['krokfmt'],
\}

" Enable fix on save
let g:ale_fix_on_save = 1
```

### Manual Integration

```vim
" Format current file
nnoremap <leader>f :!krokfmt %<CR>

" Auto-format on save
autocmd BufWritePre *.ts,*.tsx !krokfmt %
```

## IntelliJ IDEA / WebStorm

### File Watcher

1. Go to **Settings** → **Tools** → **File Watchers**
2. Click **+** to add a new watcher
3. Configure:
   - Name: `krokfmt`
   - File type: `TypeScript` and `TypeScript JSX`
   - Program: `/path/to/krokfmt`
   - Arguments: `$FilePath$`
   - Output paths: `$FilePath$`

### External Tools

1. Go to **Settings** → **Tools** → **External Tools**
2. Click **+** to add a new tool
3. Configure:
   - Name: `krokfmt`
   - Program: `/path/to/krokfmt`
   - Arguments: `$FilePath$`
   - Working directory: `$ProjectFileDir$`

Add keyboard shortcut in **Settings** → **Keymap**

## Sublime Text

### Using Sublime-Format

1. Install Package Control
2. Install `Formatter` package
3. Add to user settings:

```json
{
  "formatters": {
    "krokfmt": {
      "executable": "/path/to/krokfmt",
      "args": ["--stdout"],
      "syntax": ["TypeScript", "TypeScriptReact"]
    }
  },
  "format_on_save": {
    "TypeScript": "krokfmt",
    "TypeScriptReact": "krokfmt"
  }
}
```

## Emacs

### Using reformatter.el

```elisp
(require 'reformatter)

(reformatter-define krokfmt
  :program "krokfmt"
  :args '("--stdout"))

;; Format on save
(add-hook 'typescript-mode-hook #'krokfmt-on-save-mode)
(add-hook 'tsx-mode-hook #'krokfmt-on-save-mode)
```

### Manual Integration

```elisp
(defun krokfmt-buffer ()
  "Format current buffer with krokfmt."
  (interactive)
  (shell-command-on-region
   (point-min) (point-max)
   "krokfmt --stdout"
   t t))

;; Bind to key
(global-set-key (kbd "C-c f") 'krokfmt-buffer)
```

## Atom

### Using atom-beautify

1. Install `atom-beautify` package
2. Add to `config.cson`:

```json
{
  "atom-beautify": {
    "typescript": {
      "beautify_on_save": true,
      "default_beautifier": "krokfmt"
    },
    "executables": {
      "krokfmt": {
        "path": "/path/to/krokfmt"
      }
    }
  }
}
```

## Nova

### Using Format on Save

1. Install the "Format on Save" extension
2. Go to **Extension Settings**
3. Add formatter:
   - Command: `/path/to/krokfmt`
   - Arguments: `$FilePath`
   - File types: `ts,tsx`

## General Integration

### Using Language Server Protocol

krokfmt can be integrated with any editor supporting LSP:

```bash
# Start krokfmt in LSP mode (future feature)
krokfmt --lsp
```

### Using stdin/stdout

Most editors support formatters that read from stdin:

```bash
# Format stdin and output to stdout
echo "const x={a:1}" | krokfmt --stdin
```

## CI Integration

### Pre-commit

`.pre-commit-config.yaml`:

```yaml
repos:
  - repo: local
    hooks:
      - id: krokfmt
        name: krokfmt
        entry: krokfmt
        language: system
        files: \.(ts|tsx)$
```

### Husky

`.husky/pre-commit`:

```bash
#!/bin/sh
. "$(dirname "$0")/_/husky.sh"

npx lint-staged
```

`package.json`:

```json
{
  "lint-staged": {
    "*.{ts,tsx}": ["krokfmt"]
  }
}
```

## Troubleshooting

### Editor Can't Find krokfmt

1. Ensure krokfmt is in your PATH
2. Use absolute path in editor config
3. Restart editor after installation

### Format on Save Not Working

1. Check file associations (`.ts`, `.tsx`)
2. Verify editor has write permissions
3. Check editor's error console

### Performance Issues

1. Exclude `node_modules` and build directories
2. Format specific directories instead of entire project
3. Use editor's native format-on-save instead of file watchers