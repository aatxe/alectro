//! Textual UI View for IRC client.
use std::io;
use std::io::prelude::*;
use std::io::{Stdout, stdout};
use std::sync::Mutex;

use irc::proto::{Command, Message};
use termion;
use termion::{color, cursor, style};
use termion::cursor::DetectCursorPos;
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

pub struct View {
    buffer: Mutex<Vec<Message>>,
    // _stdout: MouseTerminal<RawTerminal<Stdout>>,
    _stdout: MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>,
}

impl View {
    pub fn new() -> io::Result<View> {
        Ok(View {
            buffer: Mutex::new(vec![]),
            // _stdout: MouseTerminal::from(stdout().into_raw_mode()?),
            _stdout: MouseTerminal::from(AlternateScreen::from(stdout().into_raw_mode()?)),
        })
    }

    pub fn push(&self, msg: Message) {
        self.buffer.lock().unwrap().push(msg);
    }

    pub fn draw(&self) -> io::Result<()> {
        let buf = self.buffer.lock().unwrap();
        let mut to_draw: Vec<_> = buf.iter().filter(|m| m.drawable()).collect();
        to_draw.reverse();

        let mut stdout = stdout();

        write!(stdout, "{}{}", termion::clear::All, cursor::Goto(1, self.lines()))?;
        for msg in &to_draw {
            msg.draw()?;
            write!(stdout, "{}\r", cursor::Up(1))?;
        }

        stdout.flush()
    }

    fn lines(&self) -> u16 {
        termion::terminal_size().unwrap_or((0, 0)).1
    }

    fn columns(&self) -> u16 {
        termion::terminal_size().unwrap_or((0, 0)).0
    }
}

pub trait Draw {
    fn draw(&self) -> io::Result<()>;

    fn draw_at_point(&self, point: (u16, u16)) -> io::Result<()> {
        write!(stdout(), "{}", cursor::Goto(point.0, point.1))?;
        self.draw()
    }

    fn drawable(&self) -> bool {
        true
    }
}

impl Draw for Message {
    fn draw(&self) -> io::Result<()> {
        let mut stdout = stdout();

        match self.command {
            Command::PRIVMSG(_, ref msg) => {
                write!(stdout, "{}: {}", self.source_nickname().unwrap_or("DEFAULT"), msg)?
            }
            Command::NOTICE(_, ref msg) => {
                write!(stdout, "{}{}: {}{}",
                       color::Bg(color::Yellow),
                       self.source_nickname().unwrap_or("DEFAULT"),
                       msg,
                       color::Bg(color::Reset),
                )?
            }
            Command::JOIN(ref chan, _, _) => {
                write!(stdout, "{}{} joined {}.{}",
                       color::Fg(color::LightBlack),
                       self.source_nickname().unwrap_or("DEFAULT"),
                       chan,
                       color::Fg(color::Reset),
                )?
            }
            Command::PART(ref chan, _) => {
                write!(stdout, "{}{} left {}.{}",
                       color::Fg(color::LightBlack),
                       self.source_nickname().unwrap_or("DEFAULT"),
                       chan,
                       color::Fg(color::Reset),
                )?
            }
            _ => (),
        }

        Ok(())
    }

    fn drawable(&self) -> bool {
        match self.command {
            Command::PRIVMSG(_, _) |
            // Command::NOTICE(_, _)  | // notices are frequently too long, we need actual wrapping
            Command::JOIN(_, _, _) |
            Command::PART(_, _) => true,
            _ => false
        }
    }
}
