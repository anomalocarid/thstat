#[derive(Default)]
pub struct MainApp {
    pub main_window: nwg::Window,
    pub file_menu: nwg::Menu,
    pub file_exit_menu: nwg::MenuItem,
    pub help_menu: nwg::Menu,
    pub help_about_menu: nwg::MenuItem,
}

impl MainApp {
    pub fn on_close(&self) {
        nwg::stop_thread_dispatch();
    }

    pub fn on_help_about(&self) {
        nwg::modal_info_message(&self.main_window, "About thstat", "About me.");
    }
}
