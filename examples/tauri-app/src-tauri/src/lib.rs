// Learn more about Tauri commands at https://v2.tauri.app/develop/calling-rust/#commands
mod mcp;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![mcp::mcpstart, mcp::mcpmessage, mcp::mcpstop])
        .plugin(tauri_plugin_mcp_transport::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
