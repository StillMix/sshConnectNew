use serde::{Deserialize, Serialize};
use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;
use tauri::command;

#[derive(Debug, Serialize, Deserialize)]
pub struct SshConnectionInfo {
    pub username: String,
    pub host: String,
    pub password: String,
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

#[command]
pub fn delete_file(connection_info: SshConnectionInfo, file_path: String) -> Result<String, String> {
    let sess = create_ssh_session(&connection_info)?;
    
    let mut channel = sess.channel_session()
        .map_err(|e| format!("Ошибка создания канала: {}", e))?;

    let command = format!("rm '{}'", file_path.replace("'", "'\"'\"'"));
    
    channel.exec(&command)
        .map_err(|e| format!("Ошибка выполнения команды: {}", e))?;

    let mut output = String::new();
    channel.read_to_string(&mut output)
        .map_err(|e| format!("Ошибка чтения вывода: {}", e))?;

    channel.wait_close()
        .map_err(|e| format!("Ошибка закрытия канала: {}", e))?;

    let exit_status = channel.exit_status().unwrap_or(-1);
    
    if exit_status != 0 {
        return Err("Ошибка удаления файла".to_string());
    }

    Ok("Файл успешно удален".to_string())
}

#[command]
pub fn delete_directory(connection_info: SshConnectionInfo, dir_path: String) -> Result<String, String> {
    let sess = create_ssh_session(&connection_info)?;
    
    let mut channel = sess.channel_session()
        .map_err(|e| format!("Ошибка создания канала: {}", e))?;

    let command = format!("rm -rf '{}'", dir_path.replace("'", "'\"'\"'"));
    
    channel.exec(&command)
        .map_err(|e| format!("Ошибка выполнения команды: {}", e))?;

    let mut output = String::new();
    channel.read_to_string(&mut output)
        .map_err(|e| format!("Ошибка чтения вывода: {}", e))?;

    channel.wait_close()
        .map_err(|e| format!("Ошибка закрытия канала: {}", e))?;

    let exit_status = channel.exit_status().unwrap_or(-1);
    
    if exit_status != 0 {
        return Err("Ошибка удаления папки".to_string());
    }

    Ok("Папка успешно удалена".to_string())
}

#[command]
pub fn rename_file(connection_info: SshConnectionInfo, old_path: String, new_path: String) -> Result<String, String> {
    let sess = create_ssh_session(&connection_info)?;
    
    let mut channel = sess.channel_session()
        .map_err(|e| format!("Ошибка создания канала: {}", e))?;

    let command = format!("mv '{}' '{}'", 
        old_path.replace("'", "'\"'\"'"), 
        new_path.replace("'", "'\"'\"'")
    );
    
    channel.exec(&command)
        .map_err(|e| format!("Ошибка выполнения команды: {}", e))?;

    let mut output = String::new();
    channel.read_to_string(&mut output)
        .map_err(|e| format!("Ошибка чтения вывода: {}", e))?;

    channel.wait_close()
        .map_err(|e| format!("Ошибка закрытия канала: {}", e))?;

    let exit_status = channel.exit_status().unwrap_or(-1);
    
    if exit_status != 0 {
        return Err("Ошибка переименования".to_string());
    }

    Ok("Переименование выполнено успешно".to_string())
}