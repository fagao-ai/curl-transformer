use arboard::Clipboard;
use notify_rust::Notification;
use regex::Regex;
use std::{thread, time::Duration};

mod config;
use config::Config;

fn main() {
    // 1. 加载配置文件，如果不存在则创建
    let config = match Config::load() {
        Ok(c) => c,
        Err(_) => {
            // 配置文件不存在，尝试创建默认配置
            println!("📝 配置文件不存在，正在创建示例配置...");

            match Config::create_default_config() {
                Ok(path) => {
                    println!("✅ 配置文件已创建！");
                    println!("📍 位置: {}", path.display());
                    println!("\n请编辑配置文件以添加你的替换规则，然后重新运行程序。");
                    println!("\n示例配置格式：");
                    println!("[[replacements]]");
                    println!("source_host = \"https://api.example.com\"");
                    println!("dest_host = \"http://localhost:3000\"");
                    return;
                }
                Err(e) => {
                    eprintln!("❌ 创建配置文件失败: {}", e);
                    return;
                }
            }
        }
    };

    // 2. 初始化剪贴板
    println!("--------------------------------------------------");
    println!("🚀 cURL 链接替换服务已启动");
    println!("👀 正在监控剪贴板...");
    println!("🔄 配置的替换规则:");
    for (i, replacement) in config.replacements.iter().enumerate() {
        println!("   {}. '{}' -> '{}'", i + 1, replacement.source_host, replacement.dest_host);
    }
    println!("--------------------------------------------------");

    let mut clipboard = match Clipboard::new() {
        Ok(cb) => cb,
        Err(e) => {
            eprintln!("Failed to initialize clipboard: {}", e);
            return;
        }
    };

    // 3. 为每个替换规则编译正则表达式
    let mut regex_patterns: Vec<(Regex, config::Replacement)> = Vec::new();

    for replacement in config.replacements {
        let pattern = format!(r#"^curl\s+['"]?{}"#, regex::escape(&replacement.source_host));

        match Regex::new(&pattern) {
            Ok(re) => {
                regex_patterns.push((re, replacement));
            }
            Err(e) => {
                eprintln!("⚠️  无效的正则表达式模式: {} - {}", replacement.source_host, e);
            }
        }
    }

    println!("Service started. Watching clipboard for cURL commands...");

    let mut last_content = String::new();

    loop {
        if let Ok(content) = clipboard.get_text() {
            // 避免重复处理同一内容
            if content != last_content {
                let trimmed = content.trim();

                // 遍历所有替换规则，检查是否匹配
                let mut matched = false;
                for (regex, replacement) in &regex_patterns {
                    if regex.is_match(trimmed) {
                        println!("--> 检测到 cURL 命令，执行替换...");
                        println!("    源地址: {}", replacement.source_host);
                        println!("    目标地址: {}", replacement.dest_host);

                        // 执行替换
                        let new_content = content.replace(&replacement.source_host, &replacement.dest_host);

                        // 写回剪贴板
                        if let Err(e) = clipboard.set_text(new_content.clone()) {
                            eprintln!("Error writing to clipboard: {}", e);
                        } else {
                            println!("--> ✅ 替换成功!");

                            // 发送系统通知
                            let _ = Notification::new()
                                .summary("cURL 替换成功")
                                .body(&format!("{} → {}",
                                    replacement.source_host,
                                    replacement.dest_host))
                                .show();

                            // 更新缓存，防止死循环
                            last_content = new_content;
                            matched = true;
                        }
                        break;
                    }
                }

                if !matched {
                    // 内容变了但不是目标格式，更新缓存
                    last_content = content;
                }
            }
        }

        // 降低 CPU 占用
        thread::sleep(Duration::from_millis(300));
    }
}