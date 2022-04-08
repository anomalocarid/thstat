#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct MainApp {
    #[nwg_control(flags: "WINDOW | MINIMIZE_BOX | VISIBLE", title: "thstat")]
    #[nwg_events(OnWindowClose: [MainApp::on_close])]
    window: nwg::Window,
}

impl MainApp {
    fn on_close(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().expect("Failed to initialize.");

    let _app = MainApp::build_ui(Default::default()).expect("Failed to create main window.");
    nwg::dispatch_thread_events();
}
