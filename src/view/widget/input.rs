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
        let cursor = self.cursor as usize;
        let (x, y) = (self.cursor, self.buf.bound().y);
        if cursor == self.string.len() {
            self.string.push(c);
            self.buf.set(x, y, &c.to_string());
        } else {
            self.string.insert(cursor, c);
            self.buf.set_str(x, y, &self.string[cursor..]);
        }
        self.cursor += 1;
    }

    pub fn backspace(&mut self) {
        // Move the cursor back one spot.
        self.cursor -= 1;

        // Remove the character from the internal string buffer.
        self.string.remove(self.cursor as usize);

        let cursor = self.cursor as usize;
        if cursor == self.string.len() {
            // Set the character to a blank on the visual buffer.
            let (x, y) = (self.cursor, self.buf.bound().y);
            self.buf.set(x, y, &" ".to_owned());
        } else {
            // Redraw the the whole visual buffer.
            self.buf.reset();
            let y = self.buf.bound().y;
            self.buf.set_str(0, y, &self.string);
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if (self.cursor as usize) < self.string.len() {
            self.cursor += 1;
        }
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
