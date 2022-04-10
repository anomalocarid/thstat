// Definitions specific to Touhou games
use crate::process::Handle;
use std::rc::Rc;
pub mod th10;

pub static PROCESS_NAMES: [&str; 13] = [
    "東方紅魔郷.exe",
    "th07.exe",
    "th08.exe",
    "th09.exe",
    "th10.exe",
    "th11.exe",
    "th12.exe",
    "th13.exe",
    "th14.exe",
    "th15.exe",
    "th16.exe",
    "th17.exe",
    "th18.exe",
];

pub trait ThGame {
    fn get_score(&self) -> Option<u64>;
}

pub struct GameBase {
    handle: Rc<Handle>,
}
