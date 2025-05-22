use serde::{Deserialize, Serialize};
use ssh2::Session;
use std::io::{Read};
use std::net::TcpStream;
use tauri::command;

#[derive(Debug, Serialize, Deserialize)]
pub struct SshConnectionInfo {
    pub username: String,
    pub host: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileContent {
    pub content: String,
    pub is_editable: bool,
    pub file_type: String,
    pub is_symlink: bool,
    pub symlink_target: Option<String>,
    pub file_size: Option<u64>,
}


fn is_text_file(filename: &str) -> bool {
    let text_extensions = [
        "txt", "md", "js", "ts", "html", "css", "scss", "json", "xml", "yaml", "yml",
        "toml", "ini", "cfg", "conf", "log", "sh", "py", "rs", "go", "php", "cpp",
        "c", "h", "hpp", "java", "kt", "swift", "rb", "pl", "lua", "sql", "vue",
        "jsx", "tsx", "scss", "less", "styl", "mod", "env", "dockerfile", "makefile",
        "gitignore", "gitconfig", "editorconfig", "prettierrc", "eslintrc", "babelrc",
        "npmrc", "yarnrc", "nvmrc", "bashrc", "zshrc", "vimrc", "tmux", "profile",
        "bashprofile", "zshprofile", "fish", "tcsh", "csh", "ksh", "bat", "cmd",
        "ps1", "psm1", "psd1", "ps1xml", "pssc", "psrc", "psc1", "R", "r", "Rmd",
        "tex", "bib", "cls", "sty", "dtx", "ins", "ltx", "rtx", "ctx", "lco",
        "def", "fd", "bbx", "cbx", "lbx", "TeX", "LaTeX", "bibtex", "context",
        "readme", "license", "changelog", "authors", "contributors", "copying",
        "install", "news", "todo", "version", "history", "notice", "acknowledgments"
    ];
    
    if let Some(ext) = filename.split('.').last() {
        let ext_lower = ext.to_lowercase();
        text_extensions.contains(&ext_lower.as_str())
    } else {
        let filename_lower = filename.to_lowercase();
        text_extensions.contains(&filename_lower.as_str()) ||
        filename_lower.starts_with("readme") ||
        filename_lower.starts_with("license") ||
        filename_lower.starts_with("changelog") ||
        filename_lower.starts_with("makefile") ||
        filename_lower.starts_with("dockerfile") ||
        filename_lower.starts_with(".env") ||
        filename_lower.starts_with(".git")
    }
}

fn is_likely_text_file(file_path: &str, content: &str) -> bool {
    let filename = file_path.split('/').last().unwrap_or(file_path);
    
    if is_text_file(filename) {
        return true;
    }
    
    let path_lower = file_path.to_lowercase();
    let common_text_paths = [
        "/etc/", "/usr/share/", "/var/log/", "/home/", "/root/", "/opt/",
        "/usr/local/", "/srv/", "/var/www/", "/var/lib/", "/usr/bin/",
        "/usr/sbin/", "/bin/", "/sbin/"
    ];
    
    for path in &common_text_paths {
        if path_lower.starts_with(path) {
            break;
        }
    }
    
    if content.len() > 1000 {
        let sample = &content[..1000];
        let non_printable_count = sample.chars()
            .filter(|c| !c.is_ascii_graphic() && !c.is_ascii_whitespace())
            .count();
        
        return non_printable_count < 50;
    }
    
    let non_printable_count = content.chars()
        .filter(|c| !c.is_ascii_graphic() && !c.is_ascii_whitespace())
        .count();
    
    return non_printable_count < (content.len() / 10);
}


fn get_file_type(filename: &str) -> String {
    if let Some(ext) = filename.split('.').last() {
        ext.to_lowercase()
    } else {
        "unknown".to_string()
    }
}

fn create_ssh_session(connection_info: &SshConnectionInfo) -> Result<Session, String> {
    let host_string = if connection_info.host.contains('@') {
        connection_info.host.clone()
    } else {
        format!("{}@{}", connection_info.username, connection_info.host)
    };

    let parts: Vec<&str> = host_string.split('@').collect();
    
    if parts.len() != 2 {
        return Err("Неверный формат строки подключения".to_string());
    }
    
    let username = parts[0];
    let host = parts[1];

    let tcp = TcpStream::connect(format!("{}:22", host))
        .map_err(|e| format!("Ошибка подключения к серверу: {}", e))?;

    let mut sess = Session::new()
        .map_err(|e| format!("Ошибка создания SSH-сессии: {}", e))?;

    sess.set_tcp_stream(tcp);
    sess.handshake()
        .map_err(|e| format!("Ошибка при рукопожатии SSH: {}", e))?;

    sess.userauth_password(username, &connection_info.password)
        .map_err(|e| format!("Ошибка аутентификации: {}", e))?;

    Ok(sess)
}

fn check_file_info(sess: &Session, file_path: &str) -> Result<(bool, Option<String>, Option<u64>), String> {
    let mut channel = sess.channel_session()
        .map_err(|e| format!("Ошибка создания канала: {}", e))?;

    let command = format!("stat -c '%F|%s' '{}' 2>/dev/null || echo 'error'", file_path.replace("'", "'\"'\"'"));
    
    channel.exec(&command)
        .map_err(|e| format!("Ошибка выполнения команды: {}", e))?;

    let mut output = String::new();
    channel.read_to_string(&mut output)
        .map_err(|e| format!("Ошибка чтения вывода: {}", e))?;

    channel.wait_close()
        .map_err(|e| format!("Ошибка закрытия канала: {}", e))?;

    if output.trim() == "error" {
        return Ok((false, None, None));
    }

    let parts: Vec<&str> = output.trim().split('|').collect();
    if parts.len() != 2 {
        return Ok((false, None, None));
    }

    let file_type = parts[0];
    let file_size = parts[1].parse::<u64>().ok();
    
    let is_symlink = file_type.contains("symbolic link");
    
    let symlink_target = if is_symlink {
        let mut link_channel = sess.channel_session()
            .map_err(|e| format!("Ошибка создания канала для readlink: {}", e))?;
        
        let readlink_command = format!("readlink '{}'", file_path.replace("'", "'\"'\"'"));
        
        if link_channel.exec(&readlink_command).is_ok() {
            let mut link_output = String::new();
            if link_channel.read_to_string(&mut link_output).is_ok() {
                let _ = link_channel.wait_close();
                Some(link_output.trim().to_string())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    Ok((is_symlink, symlink_target, file_size))
}

fn is_binary_file(sess: &Session, file_path: &str) -> bool {
    let mut channel = match sess.channel_session() {
        Ok(ch) => ch,
        Err(_) => return true,
    };

    let command = format!("file -b --mime-type '{}'", file_path.replace("'", "'\"'\"'"));
    
    if channel.exec(&command).is_err() {
        return true;
    }

    let mut output = String::new();
    if channel.read_to_string(&mut output).is_err() {
        return true;
    }

    let _ = channel.wait_close();
    
    let mime_type = output.trim().to_lowercase();
    !mime_type.starts_with("text/") && !mime_type.contains("json") && !mime_type.contains("xml")
}

#[command]
pub fn read_file_content(connection_info: SshConnectionInfo, file_path: String) -> Result<FileContent, String> {
    let sess = create_ssh_session(&connection_info)?;
    
    let filename = file_path.split('/').last().unwrap_or(&file_path);
    let file_type = get_file_type(filename);
    
    let (is_symlink, symlink_target, file_size) = check_file_info(&sess, &file_path)?;
    
    if is_symlink {
        let target_display = symlink_target.as_ref().map(|t| t.as_str()).unwrap_or("неизвестно");
        return Ok(FileContent {
            content: format!("Символическая ссылка\nПуть: {}\nСсылается на: {}", file_path, target_display),
            is_editable: false,
            file_type,
            is_symlink: true,
            symlink_target,
            file_size,
        });
    }
    
    if let Some(size) = file_size {
        if size > 10_000_000 {
            return Ok(FileContent {
                content: format!("Файл слишком большой для просмотра\nРазмер: {} байт\nПуть: {}", size, file_path),
                is_editable: false,
                file_type,
                is_symlink: false,
                symlink_target: None,
                file_size,
            });
        }
    }

    let is_text = is_text_file(filename);
    let is_binary = if is_text { false } else { is_binary_file(&sess, &file_path) };

    if is_binary {
        let mut channel = sess.channel_session()
            .map_err(|e| format!("Ошибка создания канала: {}", e))?;

        let command = format!("file -b '{}'", file_path.replace("'", "'\"'\"'"));
        
        channel.exec(&command)
            .map_err(|e| format!("Ошибка выполнения команды: {}", e))?;

        let mut file_info = String::new();
        channel.read_to_string(&mut file_info)
            .map_err(|e| format!("Ошибка получения информации о файле: {}", e))?;

        let _ = channel.wait_close();

        return Ok(FileContent {
            content: format!("Бинарный файл\nТип: {}\nПуть: {}\nИнформация: {}", file_type, file_path, file_info.trim()),
            is_editable: false,
            file_type,
            is_symlink: false,
            symlink_target: None,
            file_size,
        });
    }

    let mut channel = sess.channel_session()
        .map_err(|e| format!("Ошибка создания канала: {}", e))?;

    let command = format!("cat '{}'", file_path.replace("'", "'\"'\"'"));
    
    channel.exec(&command)
        .map_err(|e| format!("Ошибка выполнения команды: {}", e))?;

    let mut content = String::new();
    channel.read_to_string(&mut content)
        .map_err(|e| format!("Ошибка чтения файла: {}", e))?;

    channel.wait_close()
        .map_err(|e| format!("Ошибка закрытия канала: {}", e))?;

    let exit_status = channel.exit_status().unwrap_or(-1);
    
    if exit_status != 0 {
        return Err("Ошибка чтения файла".to_string());
    }

    Ok(FileContent {
        content,
        is_editable: true,
        file_type,
        is_symlink: false,
        symlink_target: None,
        file_size,
    })
}

#[command]
pub fn save_file_content(connection_info: SshConnectionInfo, file_path: String, content: String) -> Result<String, String> {
    if !is_likely_text_file(&file_path, &content) {
        return Err("Этот тип файла нельзя редактировать".to_string());
    }

    let sess = create_ssh_session(&connection_info)?;
    
    let mut channel = sess.channel_session()
        .map_err(|e| format!("Ошибка создания канала: {}", e))?;

    let temp_file = format!("/tmp/ssh_editor_{}", std::process::id());
    let escaped_content = content.replace("'", "'\"'\"'");
    let escaped_path = file_path.replace("'", "'\"'\"'");
    let escaped_temp = temp_file.replace("'", "'\"'\"'");
    
    let command = format!(
        "echo '{}' > '{}' && cp '{}' '{}' && rm '{}'",
        escaped_content, escaped_temp, escaped_temp, escaped_path, escaped_temp
    );
    
    channel.exec(&command)
        .map_err(|e| format!("Ошибка выполнения команды: {}", e))?;

    let mut output = String::new();
    let mut stderr = String::new();
    
    if let Ok(_) = channel.read_to_string(&mut output) {}
    if let Ok(_) = channel.stderr().read_to_string(&mut stderr) {}

    channel.wait_close()
        .map_err(|e| format!("Ошибка закрытия канала: {}", e))?;

    let exit_status = channel.exit_status().unwrap_or(-1);
    
    if exit_status != 0 {
        if stderr.contains("Permission denied") || stderr.contains("permission denied") {
            let mut sudo_channel = sess.channel_session()
                .map_err(|e| format!("Ошибка создания канала для sudo: {}", e))?;
            
            let sudo_command = format!(
                "echo '{}' | sudo tee '{}' > /dev/null",
                escaped_content, escaped_path
            );
            
            sudo_channel.exec(&sudo_command)
                .map_err(|e| format!("Ошибка выполнения sudo команды: {}", e))?;
            
            let mut sudo_stderr = String::new();
            if let Ok(_) = sudo_channel.stderr().read_to_string(&mut sudo_stderr) {}
            
            sudo_channel.wait_close()
                .map_err(|e| format!("Ошибка закрытия sudo канала: {}", e))?;
            
            let sudo_exit_status = sudo_channel.exit_status().unwrap_or(-1);
            
            if sudo_exit_status != 0 {
                return Err(format!("Ошибка сохранения файла с sudo (код {}): {}", sudo_exit_status, sudo_stderr));
            }
            
            return Ok("Файл успешно сохранен с правами администратора".to_string());
        }
        
        return Err(format!("Команда завершилась с ошибкой (код {}): {}", exit_status, stderr));
    }

    Ok("Файл успешно сохранен".to_string())
}

#[command]
pub fn check_file_permissions(file_path: String) -> Result<bool, String> {
    let filename = file_path.split('/').last().unwrap_or(&file_path);
    Ok(is_text_file(filename))
}