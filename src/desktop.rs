use serde::de::DeserializeOwned;
use tauri::{AppHandle, Runtime, plugin::PluginApi};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<FfpOtaUpdater<R>> {
    Ok(FfpOtaUpdater(app.clone()))
}

/// Access to the ffp-ota-updater APIs.
pub struct FfpOtaUpdater<R: Runtime>(AppHandle<R>);

impl<R: Runtime> FfpOtaUpdater<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        Ok(PingResponse {
            value: payload.value,
        })
    }
}
