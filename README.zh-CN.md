# curl-transformer (ct)

自动监控剪贴板中的 cURL 命令，并根据配置文件自动替换主机地址。

[English README](./README.md)

## 特性

- ✅ 自动监控系统剪贴板
- ✅ 根据配置文件自动替换 cURL 命令中的主机地址
- ✅ 支持多个替换规则
- ✅ 替换成功后发送系统通知
- ✅ 跨平台支持：Windows、macOS、Linux
- ✅ 首次运行自动生成示例配置文件

## 安装

### 快速安装

**Linux/macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/fagao-ai/curl-transformer/main/scripts/install.sh | sudo sh
```

**Windows (PowerShell):**
```powershell
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/fagao-ai/curl-transformer/main/scripts/install.ps1" -OutFile "install.ps1"
.\install.ps1
```

### 手动下载

从 [GitHub Releases](https://github.com/fagao-ai/curl-transformer/releases) 下载适合你系统的二进制文件。

- **Linux**: `ct-linux-amd64`
- **macOS Intel**: `ct-darwin-amd64`
- **macOS Apple Silicon**: `ct-darwin-arm64`
- **Windows**: `ct-windows-amd64.exe`

下载后将文件重命名为 `ct`（Windows 为 `ct.exe`），然后移动到 PATH 目录即可。

### 从源码构建

```bash
# 安装 Rust (如果还没安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 克隆并构建
git clone https://github.com/fagao-ai/curl-transformer.git
cd curl-transformer
cargo build --release
sudo cp target/release/ct /usr/local/bin/
```

## 使用方法

### 首次运行

首次运行时，程序会自动创建配置文件：

```bash
ct
```

配置文件位置：

- **macOS / Linux**: `~/.ct/config.toml`
- **Windows**: `%USERPROFILE%\.ct\config.toml`

### 编辑配置文件

使用你喜欢的编辑器打开配置文件：

```bash
# macOS/Linux
vim ~/.ct/config.toml

# Windows
notepad %USERPROFILE%\.ct\config.toml
```

配置文件格式：

```toml
[[replacements]]
source_host = "https://api.example.com"
dest_host = "http://localhost:3000"

[[replacements]]
source_host = "https://another-api.example.com"
dest_host = "http://localhost:8080"
```

### 启动监控

```bash
ct
```

程序会显示所有配置的替换规则，并开始监控剪贴板：

```
--------------------------------------------------
 cURL 链接替换服务已启动
  正在监控剪贴板...
 配置的替换规则:
   1. 'https://api.example.com' -> 'http://localhost:3000'
   2. 'https://another-api.example.com' -> 'http://localhost:8080'
--------------------------------------------------
```

### 工作流程

1. 程序运行后，持续监控剪贴板
2. 当你复制包含 `source_host` 的 cURL 命令时
3. 程序自动将其替换为对应的 `dest_host`
4. 替换后的命令自动更新到剪贴板
5. 系统弹出通知提示替换成功

## 示例

### 基本示例

**配置:**
```toml
[[replacements]]
source_host = "https://api.production.com"
dest_host = "http://localhost:3000"
```

**复制的命令:**
```bash
curl 'https://api.production.com/users' -H 'Authorization: Bearer xxx'
```

**自动替换为:**
```bash
curl 'http://localhost:3000/users' -H 'Authorization: Bearer xxx'
```

### 多规则示例

**配置:**
```toml
[[replacements]]
source_host = "https://api.prod.com"
dest_host = "http://localhost:3000"

[[replacements]]
source_host = "https://cdn.prod.com"
dest_host = "http://localhost:8080"

[[replacements]]
source_host = "https://auth.prod.com"
dest_host = "http://localhost:9000"
```

匹配规则按配置顺序，第一个匹配的规则会被应用。

## 项目结构

```
curl-transformer/
├── src/
│   ├── main.rs      # 主程序逻辑
│   └── config.rs    # 配置文件处理
├── Cargo.toml
└── README.md
```

## 注意事项

- 程序会持续运行并监控剪贴板，使用 `Ctrl+C` 停止
- 配置文件使用 TOML 格式，请确保格式正确
- 每个替换规则都需要包含 `source_host` 和 `dest_host` 字段
- 替换是按照配置顺序匹配的，第一个匹配的规则会被应用

## 依赖

- `arboard` - 剪贴板操作
- `regex` - 正则表达式匹配
- `serde` & `toml` - 配置文件解析
- `dirs` - 跨平台配置目录获取
- `notify-rust` - 系统通知

## License

MIT
