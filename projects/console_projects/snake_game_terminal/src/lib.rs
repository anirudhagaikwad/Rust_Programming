pub mod keyboard;
pub mod display;

#[cfg(target_os = "windows")]
mod platform {
    pub use crate::keyboard::windows::WindowsKeyboard as KeyboardImpl;
    pub use crate::display::windows::WindowsDisplay as DisplayImpl;
}

#[cfg(target_os = "linux")]
mod platform {
    pub use crate::keyboard::unix::UnixKeyboard as KeyboardImpl;
    pub use crate::display::unix::UnixDisplay as DisplayImpl;
}

pub use platform::*;