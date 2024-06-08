use termios::*;
use std::io::{self, Read, Write};
use std::os::fd::AsRawFd;

pub trait CharInterface {
    fn init(&mut self) -> bool;
    fn de_init(&mut self) -> bool;
    fn write(&self, data: &[u8]) -> io::Result<usize>;
    fn read(&self, data: &mut [u8]) -> io::Result<usize>;
}

pub struct TermTTY {
    stdin_fd: i32,
    old_config: Termios,
}

#[allow(dead_code)]
pub struct SerialTTY {
    fd: i32,
    old_config: Termios,
}

impl TermTTY {
    pub fn get_term() -> Option<Self> {
        let fd: i32;
        let cur_config: Termios;

        fd = io::stdin().as_raw_fd();

        match Termios::from_fd(fd) {
            Ok(config) => cur_config = config,
            Err(e) => {
                print!("{0}", e);
                return None
            },
        };

        Some(Self {
            stdin_fd: fd,
            old_config: cur_config,
        })
    }
}

impl CharInterface for TermTTY {
    fn init(&mut self) -> bool {
        let mut new_config: Termios;

        new_config = self.old_config;
        new_config.c_lflag &= !(ICANON | ECHO);
        new_config.c_cc[VMIN] = 0;
        new_config.c_cc[VTIME] = 0;

        match tcsetattr(self.stdin_fd, TCSANOW, &new_config) {
            Ok(_) => (),
            Err(e) => {
                print!("{0}", e);
                return false
            },
        };

        true
    }

    fn de_init(&mut self) -> bool {

        /* To ensure the enabling of ICANON & ECHO */
        self.old_config.c_lflag |= ICANON | ECHO;

        match tcsetattr(self.stdin_fd, TCSANOW, &self.old_config) {
            Ok(_) => (),
            Err(e) => {
                print!("{0}", e);
                return false
            },
        };

        true
    }

    fn write(&self, data: &[u8]) -> io::Result<usize> {
        let num_of_write = io::stdout().write(data)?;
        io::stdout().flush().unwrap();

        Ok(num_of_write)
    }

    fn read(&self, data: &mut [u8]) -> io::Result<usize> {
        io::stdin().read(data)
    }
}