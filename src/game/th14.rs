// Structures and definitions for Touhou 14
use core::ffi::c_void;

// TODO: figure out all of these addresses
const HISCORE_ADDR: u32 = 0;
const CURRENT_SCORE_ADDR: u32 = 0;
const CURRENT_POWER_ADDR: u32 = 0;
const CURRENT_LIVES_ADDR: u32 = 0;
const CURRENT_BOMBS_ADDR: u32 = 0;
const CURRENT_GRAZE_ADDR: u32 = 0;

use super::{GameBase, ThGame};
use crate::process::ProcessHandle;
use std::rc::Rc;

pub struct Th14Game {
    base: GameBase,
}

impl Th14Game {
    pub fn new(handle: Rc<ProcessHandle>) -> Self {
        let mut base: GameBase = Default::default();
        base.handle = handle;
        Th14Game { base: base }
    }
}

impl ThGame for Th14Game {
    fn update(&mut self) {}
    fn get_hiscore(&self) -> Option<u64> {
        let hiscore = self.base.handle.read_u32(HISCORE_ADDR as *const c_void);
        hiscore.map(|x| (x as u64) * 10)
    }

    fn get_score(&self) -> Option<u64> {
        let score = self
            .base
            .handle
            .read_u32(CURRENT_SCORE_ADDR as *const c_void);
        score.map(|x| (x as u64) * 10)
    }

    fn get_power(&self) -> Option<f32> {
        let power = self
            .base
            .handle
            .read_u16(CURRENT_POWER_ADDR as *const c_void);
        power.map(|x| (x as f32) / 100.0)
    }

    fn get_lives(&self) -> Option<u32> {
        self.base
            .handle
            .read_u32(CURRENT_LIVES_ADDR as *const c_void)
    }

    fn get_bombs(&self) -> Option<u32> {
        self.base
            .handle
            .read_u32(CURRENT_BOMBS_ADDR as *const c_void)
    }

    fn get_graze(&self) -> Option<u32> {
        self.base
            .handle
            .read_u32(CURRENT_GRAZE_ADDR as *const c_void)
    }
}
