// Definitions specific to Touhou games
use crate::process::ProcessHandle;
use std::rc::Rc;
pub mod th10;
pub mod th11;
pub mod th12;
pub mod th13;
pub mod th14;
pub mod th15;
pub mod th16;
pub mod th17;
pub use self::{
    th10::Th10Game, th11::Th11Game, th12::Th12Game, th13::Th13Game, th14::Th14Game, th15::Th15Game,
    th16::Th16Game, th17::Th17Game,
};

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
    fn get_hiscore(&self) -> Option<u64>;
    fn get_score(&self) -> Option<u64>;
    fn get_power(&self) -> Option<f32>;
    fn get_lives(&self) -> Option<u32>;
    fn get_bombs(&self) -> Option<u32>;
    fn get_graze(&self) -> Option<u32>;
}

pub struct GameBase {
    handle: Rc<ProcessHandle>,
}
