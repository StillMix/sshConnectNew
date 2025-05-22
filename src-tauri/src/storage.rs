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

fn get_config_path() -> Result<PathBuf, String> {
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| "Не удалось определить домашнюю директорию")?;
    
    let mut path = PathBuf::from(home_dir);
    path.push(".ssh-connect");
    
    if !path.exists() {
        fs::create_dir_all(&path).map_err(|e| format!("Ошибка создания директории: {}", e))?;
    }
    
    path.push("servers.json");
    Ok(path)
}

#[command]
pub fn save_servers(servers: Vec<ServerConfig>) -> Result<String, String> {
    let config_path = get_config_path()?;
    
    let json_data = serde_json::to_string_pretty(&servers)
        .map_err(|e| format!("Ошибка сериализации: {}", e))?;
    
    fs::write(&config_path, json_data)
        .map_err(|e| format!("Ошибка записи файла: {}", e))?;
    
    Ok("Серверы сохранены".to_string())
}

#[command]
pub fn load_servers() -> Result<Vec<ServerConfig>, String> {
    let config_path = get_config_path()?;
    
    if !config_path.exists() {
        return Ok(vec![]);
    }
    
    let json_data = fs::read_to_string(&config_path)
        .map_err(|e| format!("Ошибка чтения файла: {}", e))?;
    
    let servers: Vec<ServerConfig> = serde_json::from_str(&json_data)
        .map_err(|e| format!("Ошибка десериализации: {}", e))?;
    
    Ok(servers)
}

#[command]
pub fn delete_server(server_id: u32) -> Result<String, String> {
    let mut servers = load_servers()?;
    servers.retain(|server| server.id != server_id);
    save_servers(servers)?;
    Ok("Сервер удален".to_string())
}