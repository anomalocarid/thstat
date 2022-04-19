// Structures and definitions for Touhou 10
use core::ffi::c_void;

const HISCORE_ADDR: u32 = 0x474c40;
const CURRENT_SCORE_ADDR: u32 = 0x474c44;
const CURRENT_POWER_ADDR: u32 = 0x474c48;
const CURRENT_LIVES_ADDR: u32 = 0x474C70;
const GAME_SPEED_ADDR: u32 = 0x476f78;

use super::{GameBase, ThGame};
use crate::process::ProcessHandle;
use std::rc::Rc;

pub struct Th10Game {
    base: GameBase,
}

impl Th10Game {
    pub fn new(handle: Rc<ProcessHandle>) -> Self {
        let mut base: GameBase = Default::default();
        base.handle = handle;
        Th10Game { base: base }
    }
}

impl ThGame for Th10Game {
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
        self.base.game_speed = self.base.handle.read_u32(GAME_SPEED_ADDR as *const c_void);
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
        None // no graze counter
    }

    fn get_game_speed(&self) -> Option<u32> {
        self.base.game_speed
    }
}
