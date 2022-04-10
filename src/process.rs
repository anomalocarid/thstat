use std::mem::size_of;
/// Code to deal with listing, opening, and reading processes in Windows for thstat
use windows::{
    core::Error,
    Win32::{
        Foundation::{CloseHandle, HANDLE},
        System::Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
            TH32CS_SNAPPROCESS,
        },
    },
};

// Wrapper for handles that need to be closed
pub struct Handle(HANDLE);
impl Drop for Handle {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            unsafe { CloseHandle(self.0) };
        }
    }
}

pub struct ProcessInfo {
    pub name: String, // process name (executable name)
}

pub fn get_process_list(list: &mut Vec<ProcessInfo>) -> Result<(), Error> {
    let handle = Handle(unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)? });
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
        };
        list.push(p);

        b = unsafe { Process32NextW(handle.0, &mut pe32) }.as_bool();

        if !b {
            break;
        }
    }

    Ok(())
}
