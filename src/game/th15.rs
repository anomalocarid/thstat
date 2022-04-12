// Structures and definitions for Touhou 15
use core::ffi::c_void;

const HISCORE_ADDR: u32 = 0x4e75bc;
const CURRENT_SCORE_ADDR: u32 = 0x4e740c;
const CURRENT_POWER_ADDR: u32 = 0x4e7440;
const CURRENT_LIVES_ADDR: u32 = 0x4e7450;
const CURRENT_BOMBS_ADDR: u32 = 0x4e745c;
const CURRENT_GRAZE_ADDR: u32 = 0x4e741c;

use super::{GameBase, ThGame};
use crate::process::ProcessHandle;
use std::rc::Rc;

pub struct Th15Game {
    base: GameBase,
}

impl Th15Game {
    pub fn new(handle: Rc<ProcessHandle>) -> Self {
        Th15Game {
            base: GameBase { handle: handle },
        }
    }
}

impl ThGame for Th15Game {
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
