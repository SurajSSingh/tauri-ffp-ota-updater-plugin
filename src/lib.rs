use tauri::{
    Manager, Runtime,
    plugin::{Builder, TauriPlugin},
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
use desktop::FfpOtaUpdater;
#[cfg(mobile)]
use mobile::FfpOtaUpdater;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the ffp-ota-updater APIs.
pub trait FfpOtaUpdaterExt<R: Runtime> {
    fn ffp_ota_updater(&self) -> &FfpOtaUpdater<R>;
}

impl<R: Runtime, T: Manager<R>> crate::FfpOtaUpdaterExt<R> for T {
    fn ffp_ota_updater(&self) -> &FfpOtaUpdater<R> {
        self.state::<FfpOtaUpdater<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("ffp-ota-updater")
        .invoke_handler(tauri::generate_handler![commands::ping])
        .setup(|app, api| {
            #[cfg(mobile)]
            let ffp_ota_updater = mobile::init(app, api)?;
            #[cfg(desktop)]
            let ffp_ota_updater = desktop::init(app, api)?;
            app.manage(ffp_ota_updater);
            Ok(())
        })
        .build()
}
