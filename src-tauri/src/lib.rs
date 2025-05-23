mod ssh;
mod listdirectory;
mod storage;
mod file;
mod file_operations;
mod connect_copy;

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
            file::check_file_permissions,
            file::save_file_content,
            file::read_file_content,
            file_operations::create_file,
            file_operations::create_directory,
            file_operations::delete_file,
            file_operations::delete_directory,
            file_operations::rename_file,
            connect_copy::transfer_file_between_servers,
            
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}