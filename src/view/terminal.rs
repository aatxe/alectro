use std::io;
use std::io::{Stdout, Write};

use termion;
use termion::cursor;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

use error;
use view::{Bound, Buffer, Widget};

pub struct Terminal {
    buf_index: usize,
    buffers: [Buffer; 2],
    stdout: AlternateScreen<RawTerminal<Stdout>>,
}

impl Terminal {
    pub fn new() -> error::Result<Terminal> {
        let (width, height) = termion::terminal_size()?;
        let term_bound = Bound::new(0, 0, width, height);

        Ok(Terminal {
            buf_index: 0,
            buffers: [Buffer::empty(term_bound), Buffer::empty(term_bound)],
            stdout: AlternateScreen::from(io::stdout().into_raw_mode()?),
        })
    }

    pub fn render<W>(&mut self, widget: &W) where W: Widget {
        widget.draw(&mut self.buffers[self.buf_index]);
    }

    pub fn draw(&mut self) -> error::Result<()> {
        let width = self.current_buf().width();

        // Draw the changes from the buffer.
        let mut buf;
        {
            let changed = self.current_buf()
                .inner()
                .iter()
                .zip(self.other_buf().inner().iter())
                .enumerate()
                .filter_map(|(i, (c, p))| if c != p {
                    let i = i as u16;
                    let x = i % width;
                    let y = i / width;
                    Some((x, y, c))
                } else {
                    None
                });
            buf = String::with_capacity(changed.size_hint().0 * 3);
            let mut last_y = 0;
            let mut last_x = 0;
            for (x, y, cell) in changed {
                if y != last_y || x != last_x + 1 {
                    buf.push_str(&format!("{}", cursor::Goto(x + 1, y + 1)));
                }
                last_x = x;
                last_y = y;
                buf.push_str(&cell.grapheme)
            }
        }
        write!(self.stdout, "{}", buf)?;

        // Swap to the other buffer.
        self.swap();

        // Flush stdout.
        self.stdout.flush()?;

        Ok(())
    }

    pub fn current_buf(&self) -> &Buffer {
        &self.buffers[self.buf_index]
    }

    fn other_buf(&self) -> &Buffer {
        &self.buffers[1 - self.buf_index]
    }

    /// Swaps between the two internal buffers.
    fn swap(&mut self) {
        self.buffers[1 - self.buf_index].reset();
        self.buf_index = 1 - self.buf_index;
    }
}
