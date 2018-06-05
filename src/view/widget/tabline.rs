use error;
use view::{Bound, Buffer, Color, Style, Widget};

const EXTRA_SIZE: u16 = 3;

pub struct TabLine {
    buf: Buffer,
    tabs: Vec<Tab>,
    curr: usize,
    cursor: u16,
}

impl TabLine {
    pub fn new(x: u16, y: u16, width: u16) -> TabLine {
        TabLine {
            buf: Buffer::empty(Bound::new(x, y, width, 1)),
            tabs: Vec::new(),
            curr: 0,
            cursor: 0,
        }
    }

    pub fn from_buffer(buf: &Buffer) -> TabLine {
        TabLine::new(buf.bound().x, buf.bound().y + buf.bound().height, buf.bound().width)
    }

    pub fn add_tab(&mut self, content: &str, current: bool) {
        let mut tab = Tab::new(self.cursor, self.buf.bound().y, content);
        if current {
            self.curr = self.tabs.len();
            tab.highlighted = true;
            self.highlight_precursor();
        }
        tab.redraw();
        tab.draw(&mut self.buf);
        self.tabs.push(tab);
        self.cursor += content.len() as u16 + EXTRA_SIZE;
    }

    /// Removes the tab with the specified title.
    ///
    /// Note: bugs may occur if this is the current tab. So, switch first! (Until this is fixed...)
    pub fn remove_tab(&mut self, content: &str) -> error::Result<()> {
        let idx = self.tabs.iter().enumerate().find(|&(_, tab)| {
            tab.content == content
        }).map(|(i, _)| i);
        match idx {
            Some(idx) => {
                if idx < self.curr {
                    self.curr -= 1;
                }
                self.tabs.remove(idx);
                self.redraw();
                Ok(())
            }
            None => Err(error::Error::TabNotFound { tab: content.to_owned() }),
        }
    }

    /// Switches to the specified tab based on its title.
    pub fn switch_to(&mut self, content: &str) -> error::Result<()> {
        let original = self.curr;
        {
            match self.tabs.iter_mut().enumerate().find(|&(_, ref tab)| {
                tab.content == content
            }) {
                Some((i, tab)) => {
                    self.curr = i;
                    tab.highlighted = true;
                    tab.redraw();
                    tab.draw(&mut self.buf);
                }
                None => return Err(error::Error::TabNotFound { tab: content.to_owned() }),
            }
        }
        if original != 0 {
            self.tabs[original - 1].before_highlighted = false;
            self.tabs[original - 1].redraw();
            self.tabs[original - 1].draw(&mut self.buf);
        }
        if original != self.curr {
            self.tabs[original].highlighted = false;
            self.tabs[original].redraw();
            self.tabs[original].draw(&mut self.buf);
        }
        self.highlight_precursor();
        Ok(())
    }

    pub fn redraw(&mut self) {
        self.buf.reset();
        self.cursor = 0;
        for (n, tab) in self.tabs.iter_mut().enumerate() {
            if n == self.curr {
                tab.before_highlighted = false;
                tab.highlighted = true;
            } else if self.curr != 0 && n == self.curr - 1 {
                tab.before_highlighted = true;
                tab.highlighted = false;
            } else {
                tab.before_highlighted = false;
                tab.highlighted = false;
            }
            tab.buf.move_x(self.cursor);
            tab.redraw();
            tab.draw(&mut self.buf);
            self.cursor += tab.content.len() as u16 + EXTRA_SIZE;
        }
    }

    fn highlight_precursor(&mut self) {
        if self.curr != 0 {
            let mut tab = &mut self.tabs[self.curr - 1];
            tab.before_highlighted = true;
            tab.redraw();
            tab.draw(&mut self.buf);
        }
    }
}

impl Widget for TabLine {
    fn draw(&self, buffer: &mut Buffer) {
        buffer.merge(&self.buf);
    }
}

struct Tab {
    buf: Buffer,
    content: String,
    highlighted: bool,
    before_highlighted: bool,
}

impl Tab {
    pub fn new(x: u16, y: u16, content: &str) -> Tab {
        Tab {
            buf: Buffer::empty(Bound::new(x, y, content.len() as u16 + EXTRA_SIZE, 1)),
            content: content.to_owned(),
            highlighted: false,
            before_highlighted: false,
        }
    }

    pub fn style(&self) -> Style {
        if self.highlighted {
            Style {
                fg: Color::Black,
                bg: Color::Magenta,
                ..Style::default()
            }
        } else {
            Style {
                fg: Color::LightWhite,
                bg: Color::Black,
                ..Style::default()
            }
        }
    }

    pub fn sep_style(&self) -> Style {
        if self.highlighted {
            Style {
                fg: Color::Magenta,
                bg: Color::Black,
                ..Style::default()
            }
        } else if self.before_highlighted {
            Style {
                fg: Color::Black,
                bg: Color::Magenta,
                ..Style::default()
            }
        } else {
            Style {
                fg: Color::LightWhite,
                bg: Color::Black,
                ..Style::default()
            }
        }
    }

    pub fn sep(&self) -> &'static str {
        if self.highlighted || self.before_highlighted {
            "\u{e0b0}"
        } else {
            "\u{e0b1}"
        }
    }

    pub fn redraw(&mut self) {
        let sep = self.sep();
        let (x, y) = (self.buf.bound().x, self.buf.bound().y);
        let (style, sep_style) = (self.style(), self.sep_style());

        self.buf.set_str_styled(x, y, " ", style);
        self.buf.set_str_styled(x + 1, y, &self.content, style);
        self.buf.set_str_styled(x + 1 + self.content.len() as u16, y, " ", style);
        self.buf.set_str_styled(x + 1 + self.content.len() as u16 + 1, y, sep, sep_style);
    }
}

impl Widget for Tab {
    fn draw(&self, buffer: &mut Buffer) {
        buffer.merge(&self.buf);
    }
}
