# Cangjie for Zed

[Cangjie](https://cangjie-lang.cn/) language support for the [Zed](https://zed.dev/) editor, including syntax highlighting, LSP integration, and Material Design file icons.

## Extensions

This repository contains two extensions:

| Extension | ID | Description |
|-----------|-----|-------------|
| **Cangjie** | `cangjie` | Language support: grammar, syntax highlighting, LSP |
| **Cangjie File Icons** | `cangjie-icon-theme` | Material Design file icons with Cangjie icon (in `icon-theme/`) |

## Features

- **Syntax highlighting** — Tree-sitter grammar (v1.0.5)
- **LSP integration** — Auto-detects `LSPServer.exe` from PATH or `CANGJIE_HOME`
- **File icons** — Material Design icons for all common file types, plus Cangjie `.cj` icon
- **Auto-indent**, bracket matching, code outline, text objects

## Requirements

- [Zed](https://zed.dev/) editor
- [Cangjie SDK 1.0.5](https://cangjie-lang.cn/download/1.0.5) — for LSP support. The installer adds `LSPServer.exe` to your PATH automatically.

## Installation

### From Zed Extensions Marketplace (recommended)

1. Open Zed, go to Extensions (`Ctrl+Shift+P` → `zed: extensions`)
2. Search for "Cangjie" and "Cangjie File Icons"
3. Click Install

### Manual (dev extension)

```bash
# Language extension
cd cangjie-zed
# In Zed: Ctrl+Shift+P → zed: install dev extension → select this directory

# Icon theme
cd icon-theme
# In Zed: Ctrl+Shift+P → zed: install dev extension → select icon-theme directory
```

### Configure Icon Theme

After installing the icon theme, add to your Zed `settings.json`:

```json
"icon_theme": {
    "dark": "Cangjie File Icons"
}
```

## Project Structure

```
├── extension.toml        # Language extension manifest
├── Cargo.toml             # Rust/WASM build config
├── src/lib.rs             # WASM extension (LSP auto-detection)
├── languages/cangjie/     # Syntax highlighting, indentation, queries
├── grammars/cangjie.wasm  # Precompiled Tree-sitter grammar
└── icon-theme/            # Icon theme extension
    ├── extension.toml
    ├── icon_themes/cangjie.json
    └── icons/             # 892 Material SVG icons
```

## Building from Source

### Grammar

```bash
# The grammar is precompiled. To rebuild:
cd grammars/cangjie
tree-sitter generate
# Zed compiles to WASM during dev extension install
```

### WASM Extension

```bash
rustup target add wasm32-wasip2
cargo build --target wasm32-wasip2 --release
```

## Publishing

This repo contains two extensions registered as a single Git submodule in [zed-industries/extensions](https://github.com/zed-industries/extensions). The `extensions.toml` entry uses the `path` field:

```toml
[cangjie]
submodule = "extensions/cangjie"
version = "1.0.5"

[cangjie-icon-theme]
submodule = "extensions/cangjie"
path = "icon-theme"
version = "1.0.5"
```

To publish a new version:

1. Update `version` in both `extension.toml` and `icon-theme/extension.toml`
2. Push changes to this repo
3. Open a PR to [zed-industries/extensions](https://github.com/zed-industries/extensions) updating the submodule commit and versions

## License

MIT
