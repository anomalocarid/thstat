#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;

use nwg::NativeUi;

mod main_app;
use main_app::MainApp;
mod main_app_ui;

fn main() {
    nwg::init().expect("Failed to initialize.");

    let mut _app = MainApp::build_ui(Default::default()).expect("Failed to create main window.");
    nwg::dispatch_thread_events();
}
