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
        Th06Game {
            base: GameBase { handle: handle },
        }
    }
}

impl ThGame for Th06Game {
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
        power.map(|x| x as f32)
    }

    fn get_lives(&self) -> Option<u32> {
        let lives = self
            .base
            .handle
            .read_u8(CURRENT_LIVES_ADDR as *const c_void);
        lives.map(|x| x as u32)
    }

    fn get_bombs(&self) -> Option<u32> {
        let bombs = self
            .base
            .handle
            .read_u8(CURRENT_BOMBS_ADDR as *const c_void);
        bombs.map(|x| x as u32)
    }

    fn get_graze(&self) -> Option<u32> {
        self.base
            .handle
            .read_u32(CURRENT_GRAZE_ADDR as *const c_void)
    }
}
