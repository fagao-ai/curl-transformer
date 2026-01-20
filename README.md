# curl-transformer (ct)

Automatically monitor clipboard for cURL commands and replace host addresses based on configuration.

[中文README](./README.zh-CN.md)

## Features

- ✅ Automatic clipboard monitoring
- ✅ Automatic host replacement in cURL commands based on config
- ✅ Support for multiple replacement rules
- ✅ System notification on successful replacement
- ✅ Cross-platform support: Windows, macOS, Linux
- ✅ Auto-generate sample config on first run

## Installation

### Quick Install

**Linux/macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/fagao-ai/curl-transformer/main/scripts/install.sh | sudo sh
```

**Windows (PowerShell):**
```powershell
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/fagao-ai/curl-transformer/main/scripts/install.ps1" -OutFile "install.ps1"
.\install.ps1
```

### Manual Download

Download the binary for your system from [GitHub Releases](https://github.com/fagao-ai/curl-transformer/releases).

- **Linux**: `ct-linux-amd64`
- **macOS Intel**: `ct-darwin-amd64`
- **macOS Apple Silicon**: `ct-darwin-arm64`
- **Windows**: `ct-windows-amd64.exe`

Rename to `ct` (or `ct.exe` on Windows) and move to a directory in your PATH.

### Build from Source

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/fagao-ai/curl-transformer.git
cd curl-transformer
cargo build --release
sudo cp target/release/ct /usr/local/bin/
```

## Usage

### First Run

On first run, the program will automatically create a config file:

```bash
ct
```

Config file location:

- **macOS / Linux**: `~/.ct/config.toml`
- **Windows**: `%USERPROFILE%\.ct\config.toml`

### Edit Config File

Open the config file with your favorite editor:

```bash
# macOS/Linux
vim ~/.ct/config.toml

# Windows
notepad %USERPROFILE%\.ct\config.toml
```

Config file format:

```toml
[[replacements]]
source_host = "https://api.example.com"
dest_host = "http://localhost:3000"

[[replacements]]
source_host = "https://another-api.example.com"
dest_host = "http://localhost:8080"
```

### Start Monitoring

```bash
ct
```

The program will display all configured replacement rules and start monitoring the clipboard:

```
--------------------------------------------------
 cURL Link Replacement Service Started
  Monitoring clipboard...
 Configured replacement rules:
   1. 'https://api.example.com' -> 'http://localhost:3000'
   2. 'https://another-api.example.com' -> 'http://localhost:8080'
--------------------------------------------------
```

### How It Works

1. Program runs and continuously monitors the clipboard
2. When you copy a cURL command containing a `source_host`
3. The program automatically replaces it with the corresponding `dest_host`
4. The replaced command is updated back to the clipboard
5. A system notification pops up indicating successful replacement

## Examples

### Basic Example

**Config:**
```toml
[[replacements]]
source_host = "https://api.production.com"
dest_host = "http://localhost:3000"
```

**Copied command:**
```bash
curl 'https://api.production.com/users' -H 'Authorization: Bearer xxx'
```

**Automatically replaced with:**
```bash
curl 'http://localhost:3000/users' -H 'Authorization: Bearer xxx'
```

### Multiple Rules Example

**Config:**
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

Rules are matched in configuration order. The first matching rule will be applied.

## Project Structure

```
curl-transformer/
├── src/
│   ├── main.rs      # Main program logic
│   └── config.rs    # Config file handling
├── Cargo.toml
└── README.md
```

## Notes

- The program runs continuously and monitors the clipboard. Use `Ctrl+C` to stop
- Config file uses TOML format. Ensure the format is correct
- Each replacement rule must include `source_host` and `dest_host` fields
- Rules are matched in configuration order; the first matching rule is applied

## Dependencies

- `arboard` - Clipboard operations
- `regex` - Regular expression matching
- `serde` & `toml` - Config file parsing
- `dirs` - Cross-platform config directory
- `notify-rust` - System notifications

## License

MIT
