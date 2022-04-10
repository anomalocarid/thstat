use crate::game::th10::Th10Game;
use crate::game::{ThGame, PROCESS_NAMES};
use crate::process::{get_process_list, open_process, Process, ProcessInfo};

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
pub struct GameInfoWindow {
    pub handle: nwg::Window,
    pub layout: nwg::GridLayout,
    pub score_label: nwg::Label,
    pub score_value: nwg::Label,
}

#[derive(Default)]
pub struct MainApp {
    pub main_window: nwg::Window,
    pub update_timer: nwg::AnimationTimer,
    pub layout: nwg::GridLayout,
    pub scan_button: nwg::Button,
    pub hook_button: nwg::Button,
    pub process_list: nwg::ListBox<ProcessInfo>,
    pub menu: MainAppMenu,
    pub process_window: ProcessWindow,
    pub game_info_window: GameInfoWindow,
    pub process: Process,
    pub game: Option<Box<dyn ThGame>>,
}

impl MainApp {
    pub fn on_close(&self) {
        nwg::stop_thread_dispatch();
    }

    pub fn on_scan(&self) {
        let mut list: Vec<ProcessInfo> = Vec::new();
        let result = get_process_list(&mut list);
        match result {
            Ok(()) => {
                self.process_list.unselect_all();
                self.process_list.clear();
                let list = list
                    .into_iter()
                    .filter(|p| PROCESS_NAMES.iter().any(|&s| s == p.name))
                    .collect();
                self.process_list.set_collection(list);
            }
            Err(e) => {
                let s = format!("{}", e);
                nwg::modal_error_message(&self.main_window, "Error", &s);
            }
        }
    }

    pub fn on_hook(&mut self) {
        let sel = self.process_list.selection();
        match sel {
            Some(i) => {
                let collection = self.process_list.collection();
                let result = open_process(&collection[i]);
                match result {
                    // Successfully gained a handle to the process with desired access
                    Ok(process) => {
                        self.process = process;
                        self.process_window
                            .id_value
                            .set_text(&format!("{}", self.process.info.pid));
                        self.process_window
                            .name_value
                            .set_text(&self.process.info.name);
                        // Check that the process is a valid game (very simple check for now)
                        match self.process.info.name.as_str() {
                            "th10.exe" => {
                                self.game_info_window.handle.set_visible(true);
                                self.game =
                                    Some(Box::new(Th10Game::new(self.process.handle.clone())));
                                self.update_timer.start();
                            }
                            _ => {
                                self.game = None;
                                self.game_info_window.handle.set_visible(false);
                                let s = format!("Unsupported game: {}", self.process.info.name);
                                nwg::modal_error_message(&self.main_window, "Error", &s);
                            }
                        }
                    }
                    Err(e) => {
                        let s = format!("{}", e);
                        nwg::modal_error_message(&self.main_window, "Error", &s);
                    }
                }
            }
            None => {
                nwg::modal_info_message(&self.main_window, "Error", "No process is selected.");
            }
        }
    }

    pub fn on_game_update(&self) {
        match &self.game {
            Some(game) => {
                if let Some(score) = game.get_score() {
                    self.game_info_window
                        .score_value
                        .set_text(&format!("{}", score));
                }
            }
            None => {}
        }
    }

    pub fn on_view_process(&self) {
        let is_visible = self.process_window.handle.visible();
        self.process_window.handle.set_visible(!is_visible);
    }

    pub fn on_help_about(&self) {
        nwg::modal_info_message(&self.main_window, "About thstat", "About me.");
    }
}
