// Windows test binaries that link the Tauri runtime crash at startup with
// STATUS_ENTRYPOINT_NOT_FOUND (0xc0000139) unless an application manifest is
// embedded, so this runtime-backed smoke test only runs on Linux and macOS.
#![cfg(not(windows))]

use tauri::test::{mock_builder, mock_context, noop_assets};
use tauri_plugin_ffp_ota_updater::{FfpOtaUpdaterExt, PingRequest};

#[test]
fn ping_echoes_value() {
    let app = mock_builder()
        .plugin(tauri_plugin_ffp_ota_updater::init())
        .build(mock_context(noop_assets()))
        .expect("failed to build app with plugin");

    let response = app
        .ffp_ota_updater()
        .ping(PingRequest {
            value: Some("hello".into()),
        })
        .expect("ping failed");

    assert_eq!(response.value.as_deref(), Some("hello"));
}
