use serde::{Deserialize, Serialize};
use ssh2::Session;
use std::net::TcpStream;
use tauri::command;

#[derive(Debug, Serialize, Deserialize)]
pub struct SshConnectionInfo {
    pub username: String,
    pub host: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub is_folder: bool,
    pub path: String,
}

#[command]
pub fn ssh_connect(connection_info: SshConnectionInfo) -> Result<String, String> {
    // Парсим строку вида "user@host" на компоненты
    let host_string = if connection_info.host.contains('@') {
        connection_info.host.clone()
    } else {
        format!("{}@{}", connection_info.username, connection_info.host)
    };

    let parts: Vec<&str> = host_string.split('@').collect();
    
    if parts.len() != 2 {
        return Err("Неверный формат строки подключения. Используйте 'user@host'".to_string());
    }
    
    let username = parts[0];
    let host = parts[1];

    // Попытка подключения к SSH-серверу
    let tcp = match TcpStream::connect(format!("{}:22", host)) {
        Ok(stream) => stream,
        Err(e) => return Err(format!("Ошибка подключения к серверу: {}", e)),
    };

    let mut sess = match Session::new() {
        Ok(session) => session,
        Err(e) => return Err(format!("Ошибка создания SSH-сессии: {}", e)),
    };

    sess.set_tcp_stream(tcp);
    
    if let Err(e) = sess.handshake() {
        return Err(format!("Ошибка при рукопожатии SSH: {}", e));
    }

    // Аутентификация с помощью пароля
    if let Err(e) = sess.userauth_password(username, &connection_info.password) {
        return Err(format!("Ошибка аутентификации: {}", e));
    }

    Ok("Успешное подключение".to_string())
}
