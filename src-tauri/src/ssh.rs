use serde::{Deserialize, Serialize};
use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;
use std::path::Path;
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

#[command]
pub fn list_directory(connection_info: SshConnectionInfo, path: String) -> Result<Vec<FileEntry>, String> {
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

    // Подключение к SSH-серверу
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

    // Открываем канал для выполнения команды
    let mut channel = match sess.channel_session() {
        Ok(channel) => channel,
        Err(e) => return Err(format!("Ошибка создания канала: {}", e)),
    };

    // Выполняем команду ls с форматированием для определения типа файла
    let command = format!("ls -la {}", path);
    
    if let Err(e) = channel.exec(&command) {
        return Err(format!("Ошибка выполнения команды: {}", e));
    }

    let mut output = String::new();
    if let Err(e) = channel.read_to_string(&mut output) {
        return Err(format!("Ошибка чтения вывода команды: {}", e));
    }

    if let Err(e) = channel.wait_close() {
        return Err(format!("Ошибка закрытия канала: {}", e));
    }

    let exit_status = channel.exit_status().unwrap_or(-1);
    
    if exit_status != 0 {
        return Err(format!("Команда завершилась с ошибкой: {}", exit_status));
    }

    // Парсинг вывода команды ls -la
    let mut entries = Vec::new();
    
    for line in output.lines().skip(1) { // Пропускаем первую строку (total)
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() >= 9 {
            let permissions = parts[0];
            let filename = parts[8..].join(" ");
            
            // Пропускаем . и ..
            if filename == "." || filename == ".." {
                continue;
            }
            
            // Проверяем первый символ прав доступа для определения типа (d - директория)
            let is_folder = permissions.starts_with('d');
            
            let full_path = if path.ends_with('/') {
                format!("{}{}", path, filename)
            } else {
                format!("{}/{}", path, filename)
            };
            
            entries.push(FileEntry {
                name: filename,
                is_folder,
                path: full_path,
            });
        }
    }

    Ok(entries)
}