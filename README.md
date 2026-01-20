# curl-transformer

自动监控剪贴板中的 cURL 命令，并根据配置文件自动替换主机地址。支持多平台（macOS、Linux、Windows）。

## 功能特性

- 📋 自动监控系统剪贴板
- 🔄 根据配置文件自动替换 cURL 命令中的主机地址
- ⚙️ 支持多个替换规则
- 🔔 替换成功后发送系统通知
- 🌍 跨平台支持（macOS、Linux、Windows）
- 🚀 首次运行自动生成示例配置文件

## 快速开始

### 1. 编译项目

```bash
cargo build --release
```

### 2. 首次运行

```bash
cargo run --release
```

首次运行时，程序会自动创建配置文件并提示你编辑配置。配置文件位置：

- **macOS / Linux**: `~/.ct/config.toml`
- **Windows**: `%USERPROFILE%\.ct\config.toml`

### 3. 编辑配置文件

使用你喜欢的编辑器打开配置文件：

```bash
# 使用 vim
vim ~/.ct/config.toml

# 或使用 VS Code
code ~/.ct/config.toml

# 或使用 nano
nano ~/.ct/config.toml
```

编辑自动生成的配置文件，添加你的替换规则：

```toml
[[replacements]]
source_host = "https://chatdoc-studio.test.paodingai.com/chatdoc-studio"
dest_host = "http://localhost:8788"

[[replacements]]
source_host = "https://api.example.com"
dest_host = "http://localhost:3000"

# 可以添加更多替换规则
[[replacements]]
source_host = "https://another-api.example.com"
dest_host = "http://localhost:8080"
```

### 4. 重新运行程序

```bash
cargo run --release
```

程序启动后会显示所有配置的替换规则，并开始监控剪贴板。

## 使用方式

1. 程序运行后，会持续监控剪贴板
2. 当你复制包含配置的 `source_host` 的 cURL 命令时
3. 程序会自动将其替换为对应的 `dest_host`
4. 替换后的命令会自动更新到剪贴板中
5. 系统会弹出通知提示替换成功

## 示例

假设你有如下配置：

```toml
[[replacements]]
source_host = "https://api.production.com"
dest_host = "http://localhost:3000"
```

当你复制以下 cURL 命令：

```bash
curl 'https://api.production.com/users' -H 'Authorization: Bearer xxx'
```

程序会自动将剪贴板内容替换为：

```bash
curl 'http://localhost:3000/users' -H 'Authorization: Bearer xxx'
```

## 注意事项

- 程序会持续运行并监控剪贴板，需要手动停止（Ctrl+C）
- 配置文件使用 TOML 格式，请确保格式正确
- 每个替换规则都需要包含 `source_host` 和 `dest_host` 字段
- 替换是按照配置顺序匹配的，第一个匹配的规则会被应用

## 依赖

- `arboard` - 剪贴板操作
- `regex` - 正则表达式匹配
- `serde` & `toml` - 配置文件解析
- `dirs` - 跨平台配置目录获取
- `notify-rust` - 系统通知
