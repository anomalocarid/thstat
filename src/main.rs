#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;

use nwg::NativeUi;

mod main_app;
use main_app::MainApp;
mod builders;
mod main_app_ui;

fn main() {
    nwg::init().expect("Failed to initialize.");

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
}
