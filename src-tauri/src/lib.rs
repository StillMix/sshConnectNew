mod ssh;
mod listdirectory;
mod storage;
mod file;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            ssh::ssh_connect,
            listdirectory::list_directory,
            storage::add_server_to_config,
            storage::update_server_in_config,
            storage::remove_server_from_config,
            storage::load_servers_from_config,
            storage::get_config_path,
            file::create_file,
            file::create_directory,
            file::check_file_permissions,
            file::save_file_content,
            file::read_file_content,
            file::create_directory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}