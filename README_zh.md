# Cangjie for Zed

[Cangjie](https://cangjie-lang.cn/) 编程语言的 [Zed](https://zed.dev/) 编辑器扩展，提供语法高亮、LSP 集成和 Material Design 文件图标。

## 扩展列表

本仓库包含两个扩展：

| 扩展 | ID | 描述 |
|-----------|-----|-------------|
| **Cangjie** | `cangjie` | 语言支持：语法解析、语法高亮、LSP |
| **Cangjie File Icons** | `cangjie-icon-theme` | Material Design 文件图标 + 仓颉图标（位于 `icon-theme/`） |

## 功能特性

- **语法高亮** — 基于 Tree-sitter 语法（v1.0.5）
- **LSP 集成** — 自动从 PATH 或 `CANGJIE_HOME` 检测 `LSPServer.exe`
- **文件图标** — Material Design 风格图标，覆盖所有常见文件类型，`.cj` 文件使用仓颉专属图标
- **自动缩进**、括号匹配、代码大纲、文本对象

## 环境要求

- [Zed](https://zed.dev/) 编辑器
- [Cangjie SDK 1.0.5](https://cangjie-lang.cn/download/1.0.5) — LSP 功能需要。安装器会自动将 `LSPServer.exe` 添加到 PATH。

## 安装

### 通过 Zed 扩展市场安装（推荐）

1. 打开 Zed，进入扩展页面（`Ctrl+Shift+P` → `zed: extensions`）
2. 搜索 "Cangjie" 和 "Cangjie File Icons"
3. 点击安装

### 手动安装（开发者模式）

```bash
# 语言扩展
cd cangjie-zed
# 在 Zed 中：Ctrl+Shift+P → zed: install dev extension → 选择此目录

# 图标主题
cd icon-theme
# 在 Zed 中：Ctrl+Shift+P → zed: install dev extension → 选择 icon-theme 目录
```

### 配置图标主题

安装图标主题后，在 Zed 的 `settings.json` 中添加：

```json
"icon_theme": {
    "dark": "Cangjie File Icons"
}
```

## 项目结构

```
├── extension.toml        # 语言扩展清单
├── Cargo.toml             # Rust/WASM 编译配置
├── src/lib.rs             # WASM 扩展（LSP 自动检测）
├── languages/cangjie/     # 语法高亮、缩进、查询规则
├── grammars/cangjie.wasm  # 预编译 Tree-sitter 语法
└── icon-theme/            # 图标主题扩展
    ├── extension.toml
    ├── icon_themes/cangjie.json
    └── icons/             # 892 个 Material SVG 图标
```

## 从源码构建

### 语法解析器

```bash
# 语法已预编译。如需重新构建：
cd grammars/cangjie
tree-sitter generate
# Zed 在安装开发扩展时自动编译为 WASM
```

### WASM 扩展

```bash
rustup target add wasm32-wasip2
cargo build --target wasm32-wasip2 --release
```

## 发布

本仓库包含两个扩展，在 [zed-industries/extensions](https://github.com/zed-industries/extensions) 中注册为单个 Git 子模块，通过 `path` 字段区分：

```toml
[cangjie]
submodule = "extensions/cangjie"
version = "1.0.5"

[cangjie-icon-theme]
submodule = "extensions/cangjie"
path = "icon-theme"
version = "1.0.5"
```

发布新版本步骤：

1. 更新 `extension.toml` 和 `icon-theme/extension.toml` 中的 `version`
2. 推送代码到此仓库
3. 向 [zed-industries/extensions](https://github.com/zed-industries/extensions) 提交 PR，更新子模块提交和版本号

## 许可证

MIT
