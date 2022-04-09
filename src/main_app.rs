#[derive(Default)]
pub struct MainAppMenu {
    pub file: nwg::Menu,
    pub file_exit: nwg::MenuItem,
    pub view: nwg::Menu,
    pub view_process: nwg::MenuItem,
    pub help: nwg::Menu,
    pub help_about: nwg::MenuItem,
}

#[derive(Default)]
pub struct ProcessWindow {
    pub handle: nwg::Window,
    pub layout: nwg::GridLayout,
    pub name_label: nwg::Label,
    pub name_value: nwg::Label,
    pub id_label: nwg::Label,
    pub id_value: nwg::Label,
}

#[derive(Default)]
pub struct MainApp {
    pub main_window: nwg::Window,
    pub layout: nwg::GridLayout,
    pub scan_button: nwg::Button,
    pub hook_button: nwg::Button,
    pub process_list: nwg::ListBox<String>,
    pub menu: MainAppMenu,
    pub process_window: ProcessWindow,
}

impl MainApp {
    pub fn on_close(&self) {
        nwg::stop_thread_dispatch();
    }

    pub fn on_scan(&self) {
        use std::mem::size_of;
        use winapi::shared::minwindef::{DWORD, TRUE};
        use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
        use winapi::um::tlhelp32::{
            CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
            TH32CS_SNAPPROCESS,
        };

        let handle = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
        if handle == INVALID_HANDLE_VALUE {
            return;
        }

        let mut pe32: PROCESSENTRY32W = Default::default();
        pe32.dwSize = size_of::<PROCESSENTRY32W>() as DWORD;
        let mut b = unsafe { Process32FirstW(handle, &mut pe32) };
        if b != TRUE {
            unsafe { CloseHandle(handle) };
            return;
        }

        loop {
            let result = String::from_utf16(&pe32.szExeFile);
            match result {
                Ok(s) => {
                    self.process_list.push(s);
                }
                Err(_e) => {}
            }

            b = unsafe { Process32NextW(handle, &mut pe32) };
            if b != TRUE {
                break;
            }
        }

        unsafe { CloseHandle(handle) };
    }

    pub fn on_view_process(&self) {
        let is_visible = self.process_window.handle.visible();
        self.process_window.handle.set_visible(!is_visible);
    }

    pub fn on_help_about(&self) {
        nwg::modal_info_message(&self.main_window, "About thstat", "About me.");
    }
}
