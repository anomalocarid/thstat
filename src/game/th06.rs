// Structures and definitions for Touhou 06
use core::ffi::c_void;

const GAME_MANAGER_ADDR: u32 = 0x69bca0;
const HISCORE_ADDR: u32 = GAME_MANAGER_ADDR + 0x0c;
const CURRENT_SCORE_ADDR: u32 = GAME_MANAGER_ADDR + 0x04;
const CURRENT_POWER_ADDR: u32 = GAME_MANAGER_ADDR + 0x1810;
const CURRENT_LIVES_ADDR: u32 = GAME_MANAGER_ADDR + 0x181a;
const CURRENT_BOMBS_ADDR: u32 = GAME_MANAGER_ADDR + 0x181b;
const CURRENT_GRAZE_ADDR: u32 = GAME_MANAGER_ADDR + 0x14;

use super::{GameBase, ThGame};
use crate::process::ProcessHandle;
use std::rc::Rc;

pub struct Th06Game {
    base: GameBase,
}

impl Th06Game {
    pub fn new(handle: Rc<ProcessHandle>) -> Self {
        let mut base: GameBase = Default::default();
        base.handle = handle;
        Th06Game { base: base }
    }
}

impl ThGame for Th06Game {
    fn update(&mut self) {
        self.base.hiscore = self
            .base
            .handle
            .read_u32(HISCORE_ADDR as *const c_void)
            .map(|x| (x as u64) * 10);
        self.base.score = self
            .base
            .handle
            .read_u32(CURRENT_SCORE_ADDR as *const c_void)
            .map(|x| (x as u64) * 10);
        self.base.power = self
            .base
            .handle
            .read_u16(CURRENT_POWER_ADDR as *const c_void)
            .map(|x| x as f32);
        self.base.lives = self
            .base
            .handle
            .read_u8(CURRENT_LIVES_ADDR as *const c_void)
            .map(|x| x as u32);
        self.base.bombs = self
            .base
            .handle
            .read_u8(CURRENT_BOMBS_ADDR as *const c_void)
            .map(|x| x as u32);
        self.base.graze = self
            .base
            .handle
            .read_u32(CURRENT_GRAZE_ADDR as *const c_void);
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
