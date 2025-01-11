use super::Keyboard;
use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
use winapi::um::fileapi::ReadFile;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::winnt::HANDLE;
use winapi::um::wincon::{ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT, ENABLE_PROCESSED_INPUT};
use std::ptr;

pub struct WindowsKeyboard {
    stdin_handle: HANDLE,
}

impl WindowsKeyboard {
    pub fn new() -> Self {
        let stdin_handle: HANDLE = unsafe { winapi::um::fileapi::CreateFileA(
            b"CONIN$\0".as_ptr() as *const i8,
            winapi::um::winnt::GENERIC_READ,
            winapi::um::winnt::FILE_SHARE_READ,
            ptr::null_mut(),
            winapi::um::fileapi::OPEN_EXISTING,
            0,
            ptr::null_mut(),
        ) };

        if stdin_handle == INVALID_HANDLE_VALUE {
            panic!("Failed to get stdin handle");
        }

        let mut mode = 0;
        unsafe {
            GetConsoleMode(stdin_handle, &mut mode);
            SetConsoleMode(stdin_handle, mode & !(ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT));
        }

        WindowsKeyboard { stdin_handle }
    }
}

impl Keyboard for WindowsKeyboard {
    fn read_input(&self) -> Option<char> {
        let mut buf = [0; 1];
        unsafe {
            ReadFile(self.stdin_handle, buf.as_mut_ptr() as *mut _, 1, ptr::null_mut(), ptr::null_mut());
        }
        Some(char::from(buf[0]))
    }
}