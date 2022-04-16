// Structures and definitions for Touhou 08
use core::ffi::c_void;

const GAME_MANAGER_ADDR: u32 = 0x160f508;
const GLOBALS_ADDR: u32 = GAME_MANAGER_ADDR + 0x8;
const CURRENT_SCORE_OFFSET: u32 = 0x08;
const HISCORE_OFFSET: u32 = 0x14;
const CURRENT_LIVES_OFFSET: u32 = 0x74;
const CURRENT_POWER_OFFSET: u32 = 0x98;
const CURRENT_BOMBS_OFFSET: u32 = 0x80;
const CURRENT_GRAZE_OFFSET: u32 = 0xc;

use super::{GameBase, ThGame};
use crate::process::ProcessHandle;
use std::rc::Rc;

pub struct Th08Game {
    base: GameBase,
    globals: Option<u32>,
    score: Option<u64>,
    hiscore: Option<u64>,
    lives: Option<u32>,
    bombs: Option<u32>,
    power: Option<f32>,
    graze: Option<u32>,
}

impl Th08Game {
    pub fn new(handle: Rc<ProcessHandle>) -> Self {
        Th08Game {
            base: GameBase { handle: handle },
            globals: None,
            score: None,
            hiscore: None,
            lives: None,
            bombs: None,
            power: None,
            graze: None,
        }
    }
}

impl ThGame for Th08Game {
    fn update(&mut self) {
        // Can't guarantee the pointer is valid until in game
        // for now, just update it each time, but I really should check the game state to determine if
        // we're actually in game
        self.globals = self.base.handle.read_u32(GLOBALS_ADDR as *const c_void);
        if let Some(globals) = self.globals {
            self.score = self
                .base
                .handle
                .read_u32((globals + CURRENT_SCORE_OFFSET) as *const c_void)
                .map(|x| (x as u64) * 10);
            self.hiscore = self
                .base
                .handle
                .read_u32((globals + HISCORE_OFFSET) as *const c_void)
                .map(|x| (x as u64) * 10);
            self.lives = self
                .base
                .handle
                .read_float((globals + CURRENT_LIVES_OFFSET) as *const c_void)
                .map(|x| (x as u32));
            self.bombs = self
                .base
                .handle
                .read_float((globals + CURRENT_BOMBS_OFFSET) as *const c_void)
                .map(|x| (x as u32));
            self.power = self
                .base
                .handle
                .read_float((globals + CURRENT_POWER_OFFSET) as *const c_void);
            self.graze = self
                .base
                .handle
                .read_u32((globals + CURRENT_GRAZE_OFFSET) as *const c_void);
        }
    }

    fn get_hiscore(&self) -> Option<u64> {
        self.hiscore
    }

    fn get_score(&self) -> Option<u64> {
        self.score
    }

    fn get_power(&self) -> Option<f32> {
        self.power
    }

    fn get_lives(&self) -> Option<u32> {
        self.lives
    }

    fn get_bombs(&self) -> Option<u32> {
        self.bombs
    }

    fn get_graze(&self) -> Option<u32> {
        self.graze
    }
}
