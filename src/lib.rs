use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::McpTransport;
#[cfg(mobile)]
use mobile::McpTransport;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the mcp-transport APIs.
pub trait McpTransportExt<R: Runtime> {
  fn mcp_transport(&self) -> &McpTransport<R>;
}

impl<R: Runtime, T: Manager<R>> crate::McpTransportExt<R> for T {
  fn mcp_transport(&self) -> &McpTransport<R> {
    self.state::<McpTransport<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("mcp-transport")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let mcp_transport = mobile::init(app, api)?;
      #[cfg(desktop)]
      let mcp_transport = desktop::init(app, api)?;
      app.manage(mcp_transport);
      Ok(())
    })
    .build()
}
