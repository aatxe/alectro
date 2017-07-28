use std::io;
use std::io::Write;

use termion::cursor;

use error;
use view::{Bound, Buffer, Widget};

pub struct Input {
    buf: Buffer,
    string: Vec<String>,
    curr: usize,
    cursor: u16,
}

impl Input {
    pub fn new(x: u16, y: u16, width: u16) -> Input {
        Input {
            buf: Buffer::empty(Bound::new(x, y, width, 1)),
            string: vec![String::new()],
            curr: 0,
            cursor: 0,
        }
    }

    pub fn from_buffer(buf: Buffer) -> Input {
        Input::new(buf.bound().x, buf.bound().y + buf.bound().height + 1, buf.bound().width)
    }

    pub fn get_content(&self) -> &str {
        &self.string[self.curr]
    }

    pub fn reset(&mut self) {
        self.string.push(String::new());
        self.buf.reset();
        self.cursor = 0;
        self.curr = self.latest();
    }

    pub fn add_char(&mut self, c: char) {
        self.before_edit();

        let cursor = self.cursor as usize;
        let (x, y) = (self.cursor, self.buf.bound().y);
        if cursor == self.string[self.curr].len() {
            self.string[self.curr].push(c);
            self.buf.set(x, y, &c.to_string());
        } else {
            self.string[self.curr].insert(cursor, c);
            self.buf.set_str(x, y, &self.string[self.curr][cursor..]);
        }
        self.cursor += 1;
    }

    pub fn backspace(&mut self) {
        self.before_edit();

        // Move the cursor back one spot.
        self.cursor -= 1;

        // Remove the character from the internal string buffer.
        self.string[self.curr].remove(self.cursor as usize);

        let cursor = self.cursor as usize;
        if cursor == self.string[self.curr].len() {
            // Set the character to a blank on the visual buffer.
            let (x, y) = (self.cursor, self.buf.bound().y);
            self.buf.set(x, y, &" ".to_owned());
        } else {
            // Redraw the the whole visual buffer.
            self.redraw();
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if (self.cursor as usize) < self.string[self.curr].len() {
            self.cursor += 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.curr > 0 {
            self.curr -= 1;
            self.redraw();
            self.bound_cursor();
        }
    }

    pub fn move_down(&mut self) {
        if self.curr < self.latest() {
            self.curr += 1;
            self.redraw();
            self.bound_cursor();
        }
    }

    pub fn draw_cursor(&self) -> error::Result<()> {
        write!(io::stdout(), "{}", cursor::Goto(self.cursor + 1, self.buf.bound().y + 1))?;
        Ok(())
    }

    /// This should be called at the beginning of any buffer-editing functions.
    /// It deals with copying the current string into the last spot of the string buffer.
    fn before_edit(&mut self) {
        let latest = self.latest();
        if self.curr < latest {
            self.string[latest] = self.string[self.curr].clone();
            self.curr = latest;
        }
    }

    fn bound_cursor(&mut self) {
        if (self.cursor as usize) > self.string[self.curr].len() {
            self.cursor = self.string[self.curr].len() as u16;
        }
    }

    fn latest(&self) -> usize {
        self.string.len() - 1
    }

    fn redraw(&mut self) {
        self.buf.reset();
        let y = self.buf.bound().y;
        self.buf.set_str(0, y, &self.string[self.curr]);
    }
}

impl Widget for Input {
    fn draw(&self, buffer: &mut Buffer) {
        buffer.merge(&self.buf);
    }
}
