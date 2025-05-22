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
}

fn is_text_file(filename: &str) -> bool {
    let text_extensions = [
        "txt", "md", "js", "ts", "html", "css", "scss", "json", "xml", "yaml", "yml",
        "toml", "ini", "cfg", "conf", "log", "sh", "py", "rs", "go", "php", "cpp",
        "c", "h", "hpp", "java", "kt", "swift", "rb", "pl", "lua", "sql", "vue",
        "jsx", "tsx", "scss", "less", "styl", "mod", "env"
    ];
    
    if let Some(ext) = filename.split('.').last() {
        text_extensions.contains(&ext.to_lowercase().as_str())
    } else {
        false
    }
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

#[command]
pub fn read_file_content(connection_info: SshConnectionInfo, file_path: String) -> Result<FileContent, String> {
    let sess = create_ssh_session(&connection_info)?;
    
    let mut channel = sess.channel_session()
        .map_err(|e| format!("Ошибка создания канала: {}", e))?;

    let filename = file_path.split('/').last().unwrap_or(&file_path);
    let is_editable = is_text_file(filename);
    let file_type = get_file_type(filename);

    if is_editable {
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
        })
    } else {
        let command = format!("file -b '{}'", file_path.replace("'", "'\"'\"'"));
        
        channel.exec(&command)
            .map_err(|e| format!("Ошибка выполнения команды: {}", e))?;

        let mut file_info = String::new();
        channel.read_to_string(&mut file_info)
            .map_err(|e| format!("Ошибка получения информации о файле: {}", e))?;

        channel.wait_close()
            .map_err(|e| format!("Ошибка закрытия канала: {}", e))?;

        Ok(FileContent {
            content: format!("Файл типа: {}\nПуть: {}\nИнформация: {}", file_type, file_path, file_info.trim()),
            is_editable: false,
            file_type,
        })
    }
}

#[command]
pub fn save_file_content(connection_info: SshConnectionInfo, file_path: String, content: String) -> Result<String, String> {
    let filename = file_path.split('/').last().unwrap_or(&file_path);
    
    if !is_text_file(filename) {
        return Err("Этот тип файла нельзя редактировать".to_string());
    }

    let sess = create_ssh_session(&connection_info)?;
    
    let mut channel = sess.channel_session()
        .map_err(|e| format!("Ошибка создания канала: {}", e))?;

    let escaped_content = content.replace("'", "'\"'\"'");
    let command = format!("echo '{}' > '{}'", escaped_content, file_path.replace("'", "'\"'\"'"));
    
    channel.exec(&command)
        .map_err(|e| format!("Ошибка выполнения команды: {}", e))?;

    let mut output = String::new();
    channel.read_to_string(&mut output)
        .map_err(|e| format!("Ошибка чтения вывода: {}", e))?;

    channel.wait_close()
        .map_err(|e| format!("Ошибка закрытия канала: {}", e))?;

    let exit_status = channel.exit_status().unwrap_or(-1);
    
    if exit_status != 0 {
        return Err("Ошибка сохранения файла".to_string());
    }

    Ok("Файл успешно сохранен".to_string())
}

#[command]
pub fn check_file_permissions( file_path: String) -> Result<bool, String> {
    let filename = file_path.split('/').last().unwrap_or(&file_path);
    Ok(is_text_file(filename))
}

#[command]
pub fn create_file(connection_info: SshConnectionInfo, file_path: String) -> Result<String, String> {
    let sess = create_ssh_session(&connection_info)?;
    
    let mut channel = sess.channel_session()
        .map_err(|e| format!("Ошибка создания канала: {}", e))?;

    let command = format!("touch '{}'", file_path.replace("'", "'\"'\"'"));
    
    channel.exec(&command)
        .map_err(|e| format!("Ошибка выполнения команды: {}", e))?;

    let mut output = String::new();
    channel.read_to_string(&mut output)
        .map_err(|e| format!("Ошибка чтения вывода: {}", e))?;

    channel.wait_close()
        .map_err(|e| format!("Ошибка закрытия канала: {}", e))?;

    let exit_status = channel.exit_status().unwrap_or(-1);
    
    if exit_status != 0 {
        return Err("Ошибка создания файла".to_string());
    }

    Ok("Файл успешно создан".to_string())
}

#[command]
pub fn create_directory(connection_info: SshConnectionInfo, dir_path: String) -> Result<String, String> {
    let sess = create_ssh_session(&connection_info)?;
    
    let mut channel = sess.channel_session()
        .map_err(|e| format!("Ошибка создания канала: {}", e))?;

    let command = format!("mkdir -p '{}'", dir_path.replace("'", "'\"'\"'"));
    
    channel.exec(&command)
        .map_err(|e| format!("Ошибка выполнения команды: {}", e))?;

    let mut output = String::new();
    channel.read_to_string(&mut output)
        .map_err(|e| format!("Ошибка чтения вывода: {}", e))?;

    channel.wait_close()
        .map_err(|e| format!("Ошибка закрытия канала: {}", e))?;

    let exit_status = channel.exit_status().unwrap_or(-1);
    
    if exit_status != 0 {
        return Err("Ошибка создания папки".to_string());
    }

    Ok("Папка успешно создана".to_string())
}