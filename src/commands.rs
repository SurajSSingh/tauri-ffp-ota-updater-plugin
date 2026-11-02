use tauri::{AppHandle, Runtime, command};

use crate::FfpOtaUpdaterExt;
use crate::Result;
use crate::models::*;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.ffp_ota_updater().ping(payload)
}
