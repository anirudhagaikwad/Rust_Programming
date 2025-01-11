#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod unix;

pub trait Display {
    fn clear_screen(&self);
    fn draw(&self, buffer: &[Vec<char>]);
}