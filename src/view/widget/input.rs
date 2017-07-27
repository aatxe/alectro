use std::io;
use std::io::Write;

use termion::cursor;

use view::{Bound, Buffer, Widget};

pub struct Input {
    buf: Buffer,
    string: String,
    cursor: u16,
}

impl Input {
    pub fn new(x: u16, y: u16, width: u16) -> Input {
        Input {
            buf: Buffer::empty(Bound::new(x, y, width, 1)),
            string: String::new(),
            cursor: 0,
        }
    }

    pub fn from_buffer(buf: Buffer) -> Input {
        Input::new(buf.bound().x, buf.bound().y + buf.bound().height + 1, buf.bound().width)
    }

    pub fn get_content(&self) -> &str {
        &self.string
    }

    pub fn reset(&mut self) {
        self.string.truncate(0);
        self.buf.reset();
        self.cursor = 0;
    }

    pub fn add_char(&mut self, c: char) {
        self.string.push(c);
        let (x, y) = (self.cursor, self.buf.bound().y);
        self.buf.set(x, y, &c.to_string());
        self.cursor += 1;
    }

    pub fn backspace(&mut self) {
        // Remove the character from the internal string buffer.
        let len = self.string.len() - 1;
        self.string.truncate(len);

        // Move the cursor back one spot.
        self.cursor -= 1;

        // Set the character to a blank on the visual buffer.
        let (x, y) = (self.cursor, self.buf.bound().y);
        self.buf.set(x, y, &" ".to_owned());
    }

    pub fn set_cursor(&self) -> io::Result<()> {
        write!(io::stdout(), "{}", cursor::Goto(self.cursor + 1, self.buf.bound().y + 1))
    }
}

impl Widget for Input {
    fn draw(&self, buffer: &mut Buffer) {
        buffer.merge(&self.buf);
    }
}
