use std::str::Chars;

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use view::{Buffer, Color, Style, Widget};

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
        let mut chars = line.chars();

        let mut skip = 0;
        let mut style = style.unwrap_or_default();
        let mut x = self.starting_x;
        let mut y = self.starting_y;

        for g in graphemes {
            // Skip any characters if necessary.
            if skip > 0 {
                let _ = chars.next();
                skip -= 1;
                continue;
            }

            if let Some(c) = chars.next() {
                // Handle color formatting.
                if c == '\x03' {
                    let mut local = chars.clone();

                    if let Some(fg) = local.next_color() {
                        style = style.fg(fg);
                        skip += 2;

                        if let Some(bg) = match local.next() {
                            Some(',') => local.next_color(),
                            _ => None,
                        } {
                            style = style.bg(bg);
                            skip += 3;
                        }
                    } else {
                        style = style.fg(Color::Reset);
                        style = style.bg(Color::Reset);
                    }

                    continue;
                }
            }

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
            self.buf.set_style(x, y, style);
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


trait CharsExt {
    fn next_color(&mut self) -> Option<Color>;
}

impl<'a> CharsExt for Chars<'a> {
    fn next_color(&mut self) -> Option<Color> {
        self.next().and_then(|c1|
            self.next().and_then(|c2| {
                let mut buf = String::with_capacity(2);
                buf.push(c1);
                buf.push(c2);
                buf.parse().ok().and_then(|n|
                    Color::from_u8(n)
                )
            })
        )
    }
}
