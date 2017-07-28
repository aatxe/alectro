use irc::client::prelude::*;
use termion::event::{Event, Key};

use error;
use view::UI;

pub struct InputController {
    irc_server: IrcServer,
    ui: UI,
}

impl InputController {
    pub fn new(irc_server: IrcServer, ui: UI) -> InputController {
        InputController {
            irc_server: irc_server,
            ui: ui,
        }
    }

    pub fn ui(&self) -> &UI {
        &self.ui
    }

    pub fn handle_event(&self, event: Event) -> error::Result<()> {
        if let Event::Key(key) = event {
            match key {
                Key::Ctrl('c') | Key::Ctrl('d') | Key::Ctrl('q') => {
                    self.irc_server.send_quit("QUIT")?;
                    panic!("User quit."); // This is a terrible way to exit.
                }
                Key::Char('\n') => {
                    let mut input = self.ui.input().unwrap();
                    self.irc_server.send_privmsg("#irc-crate", input.get_content())?;
                    self.ui.chat_buf()?.push_line(
                        &format!(
                            "{}: {}",
                            self.irc_server.config().nickname(),
                            input.get_content())
                    );
                    input.reset();
                }
                Key::Char(c) => {
                    self.ui.input()?.add_char(c);
                }
                Key::Backspace => {
                    self.ui.input()?.backspace();
                }
                Key::Left => {
                    self.ui.input()?.move_left();
                }
                Key::Right => {
                    self.ui.input()?.move_right();
                }
                Key::Up => {
                    self.ui.input()?.move_up();
                }
                Key::Down => {
                    self.ui.input()?.move_down();
                }
                _ => (),
            }
        }

        Ok(())
    }
}
