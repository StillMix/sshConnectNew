use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub id: u32,
    pub title: String,
    pub user: String,
    pub password: String,
}
#[command]
pub fn get_config_file_path() -> Result<String, String> {
    let config_path = get_config_path()?;
    Ok(config_path.to_string_lossy().to_string())
}
fn get_config_path() -> Result<PathBuf, String> {
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| "Не удалось определить домашнюю директорию")?;
    
    let mut path = PathBuf::from(home_dir);
    path.push(".ssh-connect");
    
    if !path.exists() {
        fs::create_dir_all(&path).map_err(|e| format!("Ошибка создания директории {}: {}", path.display(), e))?;
        println!("Создана директория: {:?}", path);
    }
    
    path.push("servers.json");
    println!("Путь к файлу конфигурации: {:?}", path);
    Ok(path)
}

#[command]
pub fn save_servers(servers: Vec<ServerConfig>) -> Result<String, String> {
    let config_path = get_config_path()?;
    println!("Сохранение {} серверов в {:?}", servers.len(), config_path);
    
    let json_data = serde_json::to_string_pretty(&servers)
        .map_err(|e| format!("Ошибка сериализации: {}", e))?;
    
    fs::write(&config_path, &json_data)
        .map_err(|e| format!("Ошибка записи файла {}: {}", config_path.display(), e))?;
    
    println!("Серверы успешно сохранены в файл");
    Ok(format!("Серверы сохранены в {}", config_path.display()))
}

#[command]
pub fn load_servers() -> Result<Vec<ServerConfig>, String> {
    let config_path = get_config_path()?;
    
    if !config_path.exists() {
        println!("Файл конфигурации не существует: {:?}", config_path);
        return Ok(vec![]);
    }
    
    let json_data = fs::read_to_string(&config_path)
        .map_err(|e| format!("Ошибка чтения файла {}: {}", config_path.display(), e))?;
    
    if json_data.trim().is_empty() {
        println!("Файл конфигурации пуст");
        return Ok(vec![]);
    }
    
    let servers: Vec<ServerConfig> = serde_json::from_str(&json_data)
        .map_err(|e| format!("Ошибка десериализации: {}", e))?;
    
    println!("Загружено {} серверов из файла", servers.len());
    Ok(servers)
}

#[command]
pub fn delete_server(server_id: u32) -> Result<String, String> {
    let mut servers = load_servers()?;
    let initial_count = servers.len();
    servers.retain(|server| server.id != server_id);
    
    if servers.len() == initial_count {
        return Err(format!("Сервер с ID {} не найден", server_id));
    }
    
    save_servers(servers)?;
    Ok(format!("Сервер с ID {} удален", server_id))
}