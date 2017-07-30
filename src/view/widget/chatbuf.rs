use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use view::{Buffer, Style, Widget};

pub struct ChatBuf {
    buf: Buffer,
    starting_x: u16,
    starting_y: u16,
}

impl ChatBuf {
    pub fn from_buffer(buf: Buffer) -> ChatBuf {
        ChatBuf {
            buf: buf,
            starting_x: 0,
            starting_y: 0,
        }
    }

    pub fn push_line(&mut self, line: &str, style: Option<Style>) {
        let graphemes = UnicodeSegmentation::graphemes(line, true);

        let mut x = self.starting_x;
        let mut y = self.starting_y;

        for g in graphemes {
            // On a newline, carriage return and move to the next line.
            if g == "\n" {
                x = 0;
                y += 1;
                continue;
            }

            // Handle wrapping
            if x >= self.buf.width() {
                x = 0;
                y += 1;
            }

            // Handle overflow
            if y >= self.buf.height() {
                self.buf.drop_top_line();
                y -= 1;
            }

            // Set the cell to this grapheme, set the style, and moves the pointer.
            self.buf.set(x, y, g);
            if let Some(style) = style {
                self.buf.set_style(x, y, style);
            }
            x += g.width() as u16;
        }

        self.starting_x = 0;
        self.starting_y = y + 1;
    }
}

impl Widget for ChatBuf {
    fn draw(&self, buffer: &mut Buffer) {
        buffer.merge(&self.buf)
    }
}
