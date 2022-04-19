// Interface with native-windows-gui to create the App UI
use super::*;
use native_windows_gui as nwg;
use std::cell::RefCell;
use std::rc::Rc;

pub struct MainAppUi {
    // RefCell needed to be able to mutate the MainApp during events
    pub inner: Rc<RefCell<MainApp>>,
    default_handler: RefCell<Option<nwg::EventHandler>>,
}

impl nwg::NativeUi<MainAppUi> for MainApp {
    fn build_ui(mut data: MainApp) -> Result<MainAppUi, nwg::NwgError> {
        // Windows
        builders::build_main_window(&mut data)?;
        builders::build_process_window(&data.main_window, &mut data.process_window)?;
        builders::build_timing_window(&data.main_window, &mut data.timing_window)?;
        // Main menu
        builders::build_menu(&mut data)?;
        // Initialize
        data.clear_game_info();

        // Make UI struct
        let ui = MainAppUi {
            inner: Rc::new(RefCell::new(data)),
            default_handler: Default::default(),
        };
        // Event handlers
        let evt_ui = Rc::downgrade(&ui.inner);
        let handle_events = move |evt, _evt_data, handle| {
            if let Some(ui) = evt_ui.upgrade() {
                match evt {
                    nwg::Event::OnWindowClose => {
                        let mut ui = ui.borrow_mut();
                        if &handle == &ui.main_window {
                            MainApp::on_close(&mut ui);
                        }
                    }
                    nwg::Event::OnMenuItemSelected => {
                        let mut ui = ui.borrow_mut();
                        if &handle == &ui.menu.file_exit {
                            MainApp::on_close(&mut ui);
                        } else if &handle == &ui.menu.view_timing {
                            MainApp::on_view_timing(&ui);
                        } else if &handle == &ui.menu.view_process {
                            MainApp::on_view_process(&ui);
                        } else if &handle == &ui.menu.help_about {
                            MainApp::on_help_about(&ui);
                        }
                    }
                    nwg::Event::OnTimerTick => {
                        let mut ui = ui.borrow_mut();
                        if &handle == &ui.update_timer {
                            MainApp::on_game_update(&mut ui);
                        }
                    }
                    _ => {}
                }
            }
        };

        *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(
            &ui.inner.borrow().main_window.handle,
            handle_events,
        ));

        Ok(ui)
    }
}
