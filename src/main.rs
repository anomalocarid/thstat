#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;

use nwg::NativeUi;

mod main_app;
use main_app::MainApp;
mod builders;
mod game;
mod main_app_ui;
mod process;

fn main() -> windows::core::Result<()> {
    nwg::init().expect("Failed to initialize.");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set font.");

    let app = MainApp::build_ui(Default::default());
    match app {
        Ok(_ui) => {
            nwg::dispatch_thread_events();
        }
        Err(e) => {
            let s = format!("Failed to create window: {}", e);
            nwg::modal_error_message(nwg::ControlHandle::NoHandle, "Error", &s);
        }
    }

    Ok(())
}
