#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod unix;

pub trait Keyboard {
    fn read_input(&self) -> Option<char>;
}