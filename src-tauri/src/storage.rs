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

fn get_config_dir() -> Result<PathBuf, String> {
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| "Не удалось определить домашнюю директорию")?;
    
    let mut path = PathBuf::from(home_dir);
    path.push(".ssh-connect");
    
    if !path.exists() {
        fs::create_dir_all(&path)
            .map_err(|e| format!("Ошибка создания директории {}: {}", path.display(), e))?;
    }
    
    Ok(path)
}

fn get_config_file_path() -> Result<PathBuf, String> {
    let mut config_dir = get_config_dir()?;
    config_dir.push("servers.json");
    Ok(config_dir)
}

fn ensure_config_file_exists() -> Result<PathBuf, String> {
    let config_path = get_config_file_path()?;
    
    if !config_path.exists() {
        fs::write(&config_path, "[]")
            .map_err(|e| format!("Ошибка создания файла конфигурации: {}", e))?;
    }
    
    Ok(config_path)
}

#[command]
pub fn add_server_to_config(title: String, user: String, password: String) -> Result<ServerConfig, String> {
    let config_path = ensure_config_file_exists()?;
    
    let mut servers = load_servers_from_file()?;
    
    let new_id = servers.iter().map(|s| s.id).max().unwrap_or(0) + 1;
    
    let new_server = ServerConfig {
        id: new_id,
        title,
        user,
        password,
    };
    
    servers.push(new_server.clone());
    save_servers_to_file(&servers)?;
    
    Ok(new_server)
}

#[command]
pub fn update_server_in_config(id: u32, title: String, user: String, password: String) -> Result<ServerConfig, String> {
    let mut servers = load_servers_from_file()?;
    
    let server = servers.iter_mut()
        .find(|s| s.id == id)
        .ok_or_else(|| format!("Сервер с ID {} не найден", id))?;
    
    server.title = title;
    server.user = user;
    server.password = password;
    
    save_servers_to_file(&servers)?;
    
    Ok(server.clone())
}

#[command]
pub fn remove_server_from_config(id: u32) -> Result<String, String> {
    let mut servers = load_servers_from_file()?;
    
    let initial_len = servers.len();
    servers.retain(|s| s.id != id);
    
    if servers.len() == initial_len {
        return Err(format!("Сервер с ID {} не найден", id));
    }
    
    save_servers_to_file(&servers)?;
    Ok(format!("Сервер с ID {} удален", id))
}

#[command]
pub fn load_servers_from_config() -> Result<Vec<ServerConfig>, String> {
    load_servers_from_file()
}

#[command]
pub fn get_config_path() -> Result<String, String> {
    let config_path = get_config_file_path()?;
    Ok(config_path.to_string_lossy().to_string())
}

fn load_servers_from_file() -> Result<Vec<ServerConfig>, String> {
    let config_path = ensure_config_file_exists()?;
    
    let json_data = fs::read_to_string(&config_path)
        .map_err(|e| format!("Ошибка чтения файла конфигурации: {}", e))?;
    
    if json_data.trim().is_empty() {
        return Ok(vec![]);
    }
    
    let servers: Vec<ServerConfig> = serde_json::from_str(&json_data)
        .map_err(|e| format!("Ошибка парсинга конфигурации: {}", e))?;
    
    Ok(servers)
}

fn save_servers_to_file(servers: &[ServerConfig]) -> Result<(), String> {
    let config_path = ensure_config_file_exists()?;
    
    let json_data = serde_json::to_string_pretty(servers)
        .map_err(|e| format!("Ошибка сериализации конфигурации: {}", e))?;
    
    fs::write(&config_path, json_data)
        .map_err(|e| format!("Ошибка записи файла конфигурации: {}", e))?;
    
    Ok(())
}