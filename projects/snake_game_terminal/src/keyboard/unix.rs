use super::Keyboard;
use std::io::{self, Read};
use std::os::unix::io::AsRawFd;
use libc::{tcgetattr, tcsetattr, termios, TCSANOW, ECHO, ICANON};

pub struct UnixKeyboard;

impl UnixKeyboard {
    pub fn new() -> Self {
        let stdin = io::stdin();
        let fd = stdin.as_raw_fd();

        let mut termios = termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,
        };

        unsafe {
            tcgetattr(fd, &mut termios);
            termios.c_lflag &= !(ICANON | ECHO);
            tcsetattr(fd, TCSANOW, &termios);
        }

        UnixKeyboard
    }
}

impl Keyboard for UnixKeyboard {
    fn read_input(&self) -> Option<char> {
        let mut buf = [0; 1];
        io::stdin().read_exact(&mut buf).ok()?;
        Some(buf[0] as char)
    }
}