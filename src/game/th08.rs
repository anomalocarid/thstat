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

const ENEMY_MANAGER_ADDR: u32 = 0x577f20;
const TIMELINE_ZERO_OFFSET: u32 = 0x9DCDDC;

use super::{GameBase, ThGame};
use crate::process::ProcessHandle;
use std::rc::Rc;

pub struct Th08Game {
    base: GameBase,
    globals: Option<u32>,
}

impl Th08Game {
    pub fn new(handle: Rc<ProcessHandle>) -> Self {
        let mut base: GameBase = Default::default();
        base.handle = handle;
        Th08Game {
            base: base,
            globals: None,
        }
    }

    pub fn in_game(&self) -> bool {
        if let Some(val) = self
            .base
            .handle
            .read_u32((ENEMY_MANAGER_ADDR + TIMELINE_ZERO_OFFSET) as *const c_void)
        {
            return val != 0x0;
        }
        false
    }
}

impl ThGame for Th08Game {
    fn update(&mut self) {
        if !self.in_game() {
            self.globals = None; // Globals pointer not valid until in game
            return;
        }

        if self.globals == None {
            self.globals = self.base.handle.read_u32(GLOBALS_ADDR as *const c_void);
        } else {
            if let Some(globals) = self.globals {
                self.base.score = self
                    .base
                    .handle
                    .read_u32((globals + CURRENT_SCORE_OFFSET) as *const c_void)
                    .map(|x| (x as u64) * 10);
                self.base.hiscore = self
                    .base
                    .handle
                    .read_u32((globals + HISCORE_OFFSET) as *const c_void)
                    .map(|x| (x as u64) * 10);
                self.base.lives = self
                    .base
                    .handle
                    .read_float((globals + CURRENT_LIVES_OFFSET) as *const c_void)
                    .map(|x| (x as u32));
                self.base.bombs = self
                    .base
                    .handle
                    .read_float((globals + CURRENT_BOMBS_OFFSET) as *const c_void)
                    .map(|x| (x as u32));
                self.base.power = self
                    .base
                    .handle
                    .read_float((globals + CURRENT_POWER_OFFSET) as *const c_void);
                self.base.graze = self
                    .base
                    .handle
                    .read_u32((globals + CURRENT_GRAZE_OFFSET) as *const c_void);
            }
        }
    }

    fn get_hiscore(&self) -> Option<u64> {
        self.base.hiscore
    }

    fn get_score(&self) -> Option<u64> {
        self.base.score
    }

    fn get_power(&self) -> Option<f32> {
        self.base.power
    }

    fn get_lives(&self) -> Option<u32> {
        self.base.lives
    }

    fn get_bombs(&self) -> Option<u32> {
        self.base.bombs
    }

    fn get_graze(&self) -> Option<u32> {
        self.base.graze
    }
}
