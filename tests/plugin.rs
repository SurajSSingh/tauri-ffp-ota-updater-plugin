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
