// Structures and definitions for Touhou 11
use core::ffi::c_void;

const HISCORE_ADDR: u32 = 0x4a56e0;
const CURRENT_SCORE_ADDR: u32 = 0x4a56e4;
const CURRENT_POWER_ADDR: u32 = 0x4a56e8;
const CURRENT_LIVES_ADDR: u32 = 0x4a5718;
const CURRENT_GRAZE_ADDR: u32 = 0x4A5754;

use super::{GameBase, ThGame};
use crate::process::ProcessHandle;
use std::rc::Rc;

pub struct Th11Game {
    base: GameBase,
}

impl Th11Game {
    pub fn new(handle: Rc<ProcessHandle>) -> Self {
        let mut base: GameBase = Default::default();
        base.handle = handle;
        Th11Game { base: base }
    }
}

impl ThGame for Th11Game {
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
            .map(|x| (x as f32) * 5.0 / 100.0);
        self.base.lives = self
            .base
            .handle
            .read_u32(CURRENT_LIVES_ADDR as *const c_void);
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
        None // bombs are tied to power
    }

    fn get_graze(&self) -> Option<u32> {
        self.base.graze
    }
}
