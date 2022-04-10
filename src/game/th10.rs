// Structures and definitions for Touhou 10
use core::ffi::c_void;
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;

const CURRENT_SCORE_ADDR: u32 = 0x474c44;
const CURRENT_POWER_ADDR: u32 = 0x474c48;
const CURRENT_LIVES_ADDR: u32 = 0x474C70;

use super::{GameBase, ThGame};
use crate::process::Handle;
use std::rc::Rc;

pub struct Th10Game {
    base: GameBase,
}

impl Th10Game {
    pub fn new(handle: Rc<Handle>) -> Self {
        Th10Game {
            base: GameBase { handle: handle },
        }
    }
}

impl ThGame for Th10Game {
    fn get_score(&self) -> Option<u64> {
        let mut score: u32 = 0;
        let mut amt_read: usize = 0;
        let b = unsafe {
            ReadProcessMemory(
                self.base.handle.0,
                CURRENT_SCORE_ADDR as *const c_void,
                &mut score as *mut u32 as *mut c_void,
                4,
                &mut amt_read,
            )
        };

        if !b.as_bool() || amt_read != 4 {
            return None;
        }
        Some((score as u64) * 10)
    }
}
