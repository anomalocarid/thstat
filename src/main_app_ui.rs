// Interface with native-windows-gui to create the App UI
use super::*;
use native_windows_gui as nwg;
use std::cell::RefCell;
use std::rc::Rc;

pub struct MainAppUi {
    inner: Rc<MainApp>,
    default_handler: RefCell<Option<nwg::EventHandler>>,
}

impl nwg::NativeUi<MainAppUi> for MainApp {
    fn build_ui(mut data: MainApp) -> Result<MainAppUi, nwg::NwgError> {
        // Main window
        nwg::Window::builder()
            .flags(
                nwg::WindowFlags::WINDOW
                    | nwg::WindowFlags::VISIBLE
                    | nwg::WindowFlags::MINIMIZE_BOX,
            )
            .title("thstat")
            .size((200, 300))
            .build(&mut data.main_window)?;
        // Main menu
        nwg::Menu::builder()
            .text("&File")
            .disabled(false)
            .parent(&data.main_window)
            .build(&mut data.file_menu)?;
        nwg::MenuItem::builder()
            .text("E&xit")
            .disabled(false)
            .parent(&data.file_menu)
            .build(&mut data.file_exit_menu)?;
        nwg::Menu::builder()
            .text("&Help")
            .disabled(false)
            .parent(&data.main_window)
            .build(&mut data.help_menu)?;
        nwg::MenuItem::builder()
            .text("&About")
            .disabled(false)
            .parent(&data.help_menu)
            .build(&mut data.help_about_menu)?;

        // Make UI struct
        let ui = MainAppUi {
            inner: Rc::new(data),
            default_handler: Default::default(),
        };
        // Event handlers
        let evt_ui = Rc::downgrade(&ui.inner);
        let handle_events = move |evt, _evt_data, handle| {
            if let Some(ui) = evt_ui.upgrade() {
                match evt {
                    nwg::Event::OnWindowClose => {
                        if &handle == &ui.main_window {
                            MainApp::on_close(&ui);
                        }
                    }
                    nwg::Event::OnMenuItemSelected => {
                        if &handle == &ui.file_exit_menu {
                            MainApp::on_close(&ui);
                        } else if &handle == &ui.help_about_menu {
                            MainApp::on_help_about(&ui);
                        }
                    }
                    _ => {}
                }
            }
        };

        *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(
            &ui.inner.main_window.handle,
            handle_events,
        ));

        Ok(ui)
    }
}
