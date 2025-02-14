use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<McpTransport<R>> {
  Ok(McpTransport(app.clone()))
}

/// Access to the mcp-transport APIs.
pub struct McpTransport<R: Runtime>(AppHandle<R>);

impl<R: Runtime> McpTransport<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
