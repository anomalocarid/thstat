use crate::game::*;
use crate::process::{get_process_list, open_process, Process, ProcessInfo};

#[derive(Default)]
pub struct MainAppMenu {
    pub file: nwg::Menu,
    pub file_exit: nwg::MenuItem,
    pub view: nwg::Menu,
    pub view_timing: nwg::MenuItem,
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
pub struct TimingWindow {
    pub handle: nwg::Window,
    pub layout: nwg::GridLayout,
    pub speed_label: nwg::Label,
    pub speed_value: nwg::Label,
}

#[derive(Default)]
pub struct MainApp {
    pub main_window: nwg::Window,
    pub update_timer: nwg::AnimationTimer,
    pub status_label: nwg::Label,
    pub hiscore_label: nwg::Label,
    pub hiscore_value: nwg::Label,
    pub score_label: nwg::Label,
    pub score_value: nwg::Label,
    pub power_label: nwg::Label,
    pub power_value: nwg::Label,
    pub lives_label: nwg::Label,
    pub lives_value: nwg::Label,
    pub bombs_label: nwg::Label,
    pub bombs_value: nwg::Label,
    pub graze_label: nwg::Label,
    pub graze_value: nwg::Label,
    pub layout: nwg::GridLayout,
    pub menu: MainAppMenu,
    pub process_window: ProcessWindow,
    pub timing_window: TimingWindow,
    pub process: Process,
    pub game: Option<Box<dyn ThGame>>,
}

impl MainApp {
    pub fn on_close(&mut self) {
        self.update_timer.stop();
        self.game = None;
        nwg::stop_thread_dispatch();
    }

    pub fn clear_game_info(&mut self) {
        self.game = None;
        self.status_label.set_text("Scanning for game...");
        self.hiscore_value.set_text("");
        self.score_value.set_text("");
        self.power_value.set_text("");
        self.lives_value.set_text("");
        self.bombs_value.set_text("");
        self.graze_value.set_text("");
        self.timing_window.speed_value.set_text("");
    }

    fn init_game(&mut self, name: &str) -> bool {
        match name {
            "東方紅魔郷.exe" => {
                self.game = Some(Box::new(Th06Game::new(self.process.handle.clone())));
                self.status_label
                    .set_text("Touhou 06 - Embodiment of Scarlet Devil");
            }
            "th08.exe" => {
                self.game = Some(Box::new(Th08Game::new(self.process.handle.clone())));
                self.status_label.set_text("Touhou 08 - Imperishable Night");
            }
            "th10.exe" => {
                self.game = Some(Box::new(Th10Game::new(self.process.handle.clone())));
                self.status_label.set_text("Touhou 10 - Mountain of Faith");
            }
            "th11.exe" => {
                self.game = Some(Box::new(Th11Game::new(self.process.handle.clone())));
                self.status_label
                    .set_text("Touhou 11 - Subterranean Animism");
            }
            "th12.exe" => {
                self.game = Some(Box::new(Th12Game::new(self.process.handle.clone())));
                self.status_label
                    .set_text("Touhou 12 - Undefined Fantastic Object");
            }
            "th13.exe" => {
                self.game = Some(Box::new(Th13Game::new(self.process.handle.clone())));
                self.status_label.set_text("Touhou 13 - Ten Desires");
            }
            "th15.exe" => {
                self.game = Some(Box::new(Th15Game::new(self.process.handle.clone())));
                self.status_label
                    .set_text("Touhou 15 - Legacy of Lunatic Kingdom");
            }
            "th16.exe" => {
                self.game = Some(Box::new(Th16Game::new(self.process.handle.clone())));
                self.status_label
                    .set_text("Touhou 16 - Hidden Star in Four Seasons");
            }
            "th17.exe" => {
                self.game = Some(Box::new(Th17Game::new(self.process.handle.clone())));
                self.status_label
                    .set_text("Touhou 17 - Wily Beast and Weakest Creature");
            }
            _ => {
                return false;
            }
        }
        true
    }

    pub fn on_game_update(&mut self) {
        match &mut self.game {
            Some(game) => {
                // If process has exited, clear the game information out
                if !self.process.handle.is_active() {
                    self.clear_game_info();
                    return;
                }
                game.update();
                // Update game information
                if let Some(hiscore) = game.get_hiscore() {
                    self.hiscore_value.set_text(&format!("{}", hiscore))
                }
                if let Some(score) = game.get_score() {
                    self.score_value.set_text(&format!("{}", score));
                }
                if let Some(power) = game.get_power() {
                    self.power_value.set_text(&format!("{:.2}", power));
                }
                if let Some(lives) = game.get_lives() {
                    self.lives_value.set_text(&format!("{}", lives));
                }
                if let Some(bombs) = game.get_bombs() {
                    self.bombs_value.set_text(&format!("{}", bombs));
                }
                if let Some(graze) = game.get_graze() {
                    self.graze_value.set_text(&format!("{}", graze));
                }
                if self.timing_window.handle.visible() {
                    if let Some(game_speed) = game.get_game_speed() {
                        self.timing_window
                            .speed_value
                            .set_text(&format!("{}", game_speed));
                    }
                }
            }
            // Scan for running game and  hook the first found
            None => {
                // List of running processes
                let mut list: Vec<ProcessInfo> = Vec::new();
                let result = get_process_list(&mut list);
                match result {
                    Ok(()) => {
                        // Only filter out supported games
                        let mut list: Vec<ProcessInfo> = list
                            .into_iter()
                            .filter(|p| PROCESS_NAMES.iter().any(|&s| s == p.name))
                            .collect();
                        if let Some(p) = list.pop() {
                            // Build GameInfo object based on game name
                            let result = open_process(&p);
                            match result {
                                Ok(process) => {
                                    self.process = process;
                                    if self.init_game(p.name.as_str()) {
                                        self.process_window
                                            .id_value
                                            .set_text(&format!("{}", self.process.info.pid));
                                        self.process_window
                                            .name_value
                                            .set_text(&format!("{}", &self.process.info.name));
                                    }
                                }
                                Err(e) => {
                                    let s = format!("{}", e);
                                    nwg::modal_error_message(&self.main_window, "Error", &s);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        let s = format!("{}", e);
                        nwg::modal_error_message(&self.main_window, "Error", &s);
                    }
                }
            }
        }
    }

    pub fn toggle_window(&self, window: &nwg::Window, menu_item: &nwg::MenuItem) {
        let visibility = !window.visible();
        menu_item.set_checked(visibility);
        window.set_visible(visibility);
    }

    pub fn on_view_timing(&self) {
        self.toggle_window(&self.timing_window.handle, &self.menu.view_timing);
    }

    pub fn on_view_process(&self) {
        self.toggle_window(&self.process_window.handle, &self.menu.view_process);
    }

    pub fn on_help_about(&self) {
        nwg::modal_info_message(&self.main_window, "About thstat", "About me.");
    }
}
