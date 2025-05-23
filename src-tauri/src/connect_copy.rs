use serde::{Deserialize, Serialize};
use ssh2::Session;
use std::io::{Read, Write};
use std::net::TcpStream;
use tauri::command;

#[derive(Debug, Serialize, Deserialize)]
pub struct SshConnectionInfo {
    pub username: String,
    pub host: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileTransferRequest {
    pub source_connection: SshConnectionInfo,
    pub destination_connection: SshConnectionInfo,
    pub file_path: String,
    pub is_folder: bool,
    pub destination_path: String,
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

fn get_file_permissions(session: &Session, file_path: &str) -> Result<u32, String> {
    let mut channel = session.channel_session()
        .map_err(|e| format!("Ошибка создания канала: {}", e))?;

    let command = format!("stat -c '%a' '{}'", file_path.replace("'", "'\"'\"'"));
    
    channel.exec(&command)
        .map_err(|e| format!("Ошибка выполнения команды: {}", e))?;

    let mut output = String::new();
    channel.read_to_string(&mut output)
        .map_err(|e| format!("Ошибка чтения вывода: {}", e))?;

    channel.wait_close()
        .map_err(|e| format!("Ошибка закрытия канала: {}", e))?;

    let permissions = output.trim().parse::<u32>()
        .map_err(|_| "Ошибка парсинга прав доступа".to_string())?;

    Ok(permissions)
}

fn transfer_file_content(
    source_session: &Session,
    dest_session: &Session,
    source_path: &str,
    dest_path: &str,
) -> Result<(), String> {
    let source_sftp = source_session.sftp()
        .map_err(|e| format!("Ошибка создания SFTP канала источника: {}", e))?;
    
    let dest_sftp = dest_session.sftp()
        .map_err(|e| format!("Ошибка создания SFTP канала получателя: {}", e))?;

    let permissions = get_file_permissions(source_session, source_path).unwrap_or(0o644);

    let mut source_file = source_sftp.open(&std::path::Path::new(source_path))
        .map_err(|e| format!("Ошибка открытия исходного файла: {}", e))?;

    let mut dest_file = dest_sftp.create(&std::path::Path::new(dest_path))
        .map_err(|e| format!("Ошибка создания файла назначения: {}", e))?;

    let mut buffer = vec![0u8; 8192];
    loop {
        let bytes_read = source_file.read(&mut buffer)
            .map_err(|e| format!("Ошибка чтения из исходного файла: {}", e))?;
        
        if bytes_read == 0 {
            break;
        }
        
        dest_file.write_all(&buffer[..bytes_read])
            .map_err(|e| format!("Ошибка записи в файл назначения: {}", e))?;
    }

    drop(dest_file);
    
    dest_sftp.setstat(&std::path::Path::new(dest_path), &{
        let mut stat = ssh2::FileStat::new();
        stat.perm = Some(permissions);
        stat
    }).map_err(|e| format!("Ошибка установки прав доступа: {}", e))?;

    Ok(())
}

fn get_directory_contents(session: &Session, dir_path: &str) -> Result<Vec<(String, bool)>, String> {
    let mut channel = session.channel_session()
        .map_err(|e| format!("Ошибка создания канала: {}", e))?;

    let command = format!("ls -la '{}'", dir_path.replace("'", "'\"'\"'"));
    
    channel.exec(&command)
        .map_err(|e| format!("Ошибка выполнения команды: {}", e))?;

    let mut output = String::new();
    channel.read_to_string(&mut output)
        .map_err(|e| format!("Ошибка чтения вывода команды: {}", e))?;

    channel.wait_close()
        .map_err(|e| format!("Ошибка закрытия канала: {}", e))?;

    let exit_status = channel.exit_status().unwrap_or(-1);
    
    if exit_status != 0 {
        return Err(format!("Команда завершилась с ошибкой: {}", exit_status));
    }

    let mut entries = Vec::new();
    
    for line in output.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() >= 9 {
            let permissions = parts[0];
            let filename = parts[8..].join(" ");
            
            if filename == "." || filename == ".." {
                continue;
            }
            
            let is_folder = permissions.starts_with('d');
            entries.push((filename, is_folder));
        }
    }

    Ok(entries)
}

fn create_directory_if_not_exists(session: &Session, dir_path: &str) -> Result<(), String> {
    let sftp = session.sftp()
        .map_err(|e| format!("Ошибка создания SFTP канала: {}", e))?;
    
    match sftp.stat(&std::path::Path::new(dir_path)) {
        Ok(_) => Ok(()),
        Err(_) => {
            sftp.mkdir(&std::path::Path::new(dir_path), 0o755)
                .map_err(|e| format!("Ошибка создания директории: {}", e))
        }
    }
}

fn transfer_directory_recursive(
    source_session: &Session,
    dest_session: &Session,
    source_path: &str,
    dest_path: &str,
) -> Result<(), String> {
    create_directory_if_not_exists(dest_session, dest_path)?;

    let entries = get_directory_contents(source_session, source_path)?;
    
    for (filename, is_folder) in entries {
        let source_item_path = format!("{}/{}", source_path, filename);
        let dest_item_path = format!("{}/{}", dest_path, filename);
        
        if is_folder {
            transfer_directory_recursive(source_session, dest_session, &source_item_path, &dest_item_path)?;
        } else {
            transfer_file_content(source_session, dest_session, &source_item_path, &dest_item_path)?;
        }
    }

    Ok(())
}

#[command]
pub fn transfer_file_between_servers(transfer_request: FileTransferRequest) -> Result<String, String> {
    let source_session = create_ssh_session(&transfer_request.source_connection)?;
    let dest_session = create_ssh_session(&transfer_request.destination_connection)?;

    if transfer_request.is_folder {
        transfer_directory_recursive(
            &source_session,
            &dest_session,
            &transfer_request.file_path,
            &transfer_request.destination_path,
        )?;
        
        Ok(format!("Папка '{}' успешно скопирована", transfer_request.file_path))
    } else {
        transfer_file_content(
            &source_session,
            &dest_session,
            &transfer_request.file_path,
            &transfer_request.destination_path,
        )?;
        
        Ok(format!("Файл '{}' успешно скопирован", transfer_request.file_path))
    }
}