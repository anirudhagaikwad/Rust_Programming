use super::Display;

pub struct UnixDisplay;

impl UnixDisplay {
    pub fn new() -> Self {
        UnixDisplay
    }
}

impl Display for UnixDisplay {
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