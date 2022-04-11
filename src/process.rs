// Code to deal with listing, opening, and reading processes in Windows for thstat
use core::ffi::c_void;
use std::mem::size_of;
use std::rc::Rc;
use windows::{
    core::Error,
    Win32::{
        Foundation::{CloseHandle, BOOL, HANDLE, STILL_ACTIVE},
        System::{
            Diagnostics::{
                Debug::ReadProcessMemory,
                ToolHelp::{
                    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
                    TH32CS_SNAPPROCESS,
                },
            },
            Threading::{
                GetExitCodeProcess, OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
            },
        },
    },
};

/// Wrapper for handles to a process
#[derive(Default)]
pub struct ProcessHandle(pub HANDLE);

impl ProcessHandle {
    pub fn is_active(&self) -> bool {
        let mut exit_code: u32 = 0;
        let b = unsafe { GetExitCodeProcess(self.0, &mut exit_code as *mut u32) };
        if b.as_bool() {
            return exit_code as i32 == STILL_ACTIVE.0;
        }
        false
    }

    pub fn read_u32(&self, addr: *const c_void) -> Option<u32> {
        let mut dword: u32 = 0;
        let mut amt_read: usize = 0;
        let b = unsafe {
            ReadProcessMemory(
                self.0,
                addr,
                &mut dword as *mut u32 as *mut c_void,
                4,
                &mut amt_read,
            )
        };

        if !b.as_bool() || amt_read != 4 {
            return None;
        }

        Some(dword)
    }

    pub fn read_u16(&self, addr: *const c_void) -> Option<u32> {
        let mut word: u32 = 0;
        let mut amt_read: usize = 0;
        let b = unsafe {
            ReadProcessMemory(
                self.0,
                addr,
                &mut word as *mut u32 as *mut c_void,
                2,
                &mut amt_read,
            )
        };

        if !b.as_bool() || amt_read != 2 {
            return None;
        }

        Some(word)
    }
}

impl Drop for ProcessHandle {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            unsafe { CloseHandle(self.0) };
        }
    }
}

#[derive(Default, Clone)]
pub struct ProcessInfo {
    pub name: String, // process name (executable name)
    pub pid: u32,     // process ID
}

#[derive(Default)]
pub struct Process {
    pub handle: Rc<ProcessHandle>,
    pub info: ProcessInfo,
}

impl std::fmt::Display for ProcessInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn get_process_list(list: &mut Vec<ProcessInfo>) -> Result<(), Error> {
    let handle = ProcessHandle(unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)? });
    let mut pe32: PROCESSENTRY32W = Default::default();
    pe32.dwSize = size_of::<PROCESSENTRY32W>() as u32;
    let mut b = unsafe { Process32FirstW(handle.0, &mut pe32) }.as_bool();

    if !b {
        return Err(Error::from_win32());
    }

    loop {
        // Remove trailing zeros from the strings
        let s: Vec<u16> = pe32.szExeFile.into_iter().take_while(|&c| c != 0).collect();
        let p = ProcessInfo {
            name: String::from_utf16_lossy(&s),
            pid: pe32.th32ProcessID,
        };
        list.push(p);

        b = unsafe { Process32NextW(handle.0, &mut pe32) }.as_bool();

        if !b {
            break;
        }
    }

    Ok(())
}

pub fn open_process(p: &ProcessInfo) -> Result<Process, Error> {
    let handle =
        unsafe { OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, BOOL(0), p.pid) }?;
    Ok(Process {
        handle: Rc::new(ProcessHandle(handle)),
        info: p.clone(),
    })
}
