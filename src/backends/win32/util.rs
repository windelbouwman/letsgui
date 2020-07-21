use std::ptr::null_mut;
use winapi::shared::minwindef::*;
use winapi::um::libloaderapi;

pub fn win32_string(value: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

pub fn get_h_instance() -> HMODULE {
    unsafe { libloaderapi::GetModuleHandleW(null_mut()) }
}
