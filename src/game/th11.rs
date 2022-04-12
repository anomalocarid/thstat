// Structures and definitions for Touhou 11
use core::ffi::c_void;

//const HISCORE_ADDR: u32 = 0;
const CURRENT_SCORE_ADDR: u32 = 0x4a56e4;
const CURRENT_POWER_ADDR: u32 = 0x4a56e8;
const CURRENT_LIVES_ADDR: u32 = 0x4a5718;

use super::{GameBase, ThGame};
use crate::process::ProcessHandle;
use std::rc::Rc;

pub struct Th11Game {
    base: GameBase,
}

impl Th11Game {
    pub fn new(handle: Rc<ProcessHandle>) -> Self {
        Th11Game {
            base: GameBase { handle: handle },
        }
    }
}

impl ThGame for Th11Game {
    // TODO: figure out address for high score
    fn get_hiscore(&self) -> Option<u64> {
        None
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
        power.map(|x| (x as f32) * 5.0 / 100.0)
    }

    fn get_lives(&self) -> Option<u32> {
        self.base
            .handle
            .read_u32(CURRENT_LIVES_ADDR as *const c_void)
    }

    fn get_bombs(&self) -> Option<u32> {
        None
    }
}
