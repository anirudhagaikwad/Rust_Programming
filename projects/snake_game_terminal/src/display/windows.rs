use super::Display;

pub struct WindowsDisplay;

impl WindowsDisplay {
    pub fn new() -> Self {
        WindowsDisplay
    }
}

impl Display for WindowsDisplay {
    fn clear_screen(&self) {
        print!("{}[2J", 27 as char);
    }

    fn draw(&self, buffer: &[Vec<char>]) {
        for row in buffer {
            for &cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }
}