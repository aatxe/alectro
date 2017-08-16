use irc::client::prelude::*;
use termion::event::{Event, Key};

use error;
use model;
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
                Key::Ctrl('c') | Key::Ctrl('d') => {
                    self.irc_server.send_quit("QUIT")?;
                    bail!(error::ErrorKind::UserQuit);
                }
                Key::Char('\n') => {
                    let mut input = self.ui.input().unwrap();
                    if input.get_content().starts_with('/') {
                        let tokens: Vec<_> = input.get_content().split(' ').collect();
                        match &tokens[0][1..] {
                            "switch" => if tokens.len() >= 2 {
                                self.ui.switch_to(tokens[1])?;
                            },
                            "join" => if tokens.len() >= 2 {
                                self.irc_server.send_join(tokens[1])?;
                                self.ui.new_chat_buf(tokens[1])?;
                                self.ui.switch_to(tokens[1])?;
                            },
                            "part" => if tokens.len() >= 2 {
                                self.irc_server.send_part(tokens[1])?;
                                self.ui.remove_chat_buf(tokens[1])?;
                            },
                            "quit" => {
                                self.irc_server.send_quit("QUIT")?;
                                bail!(error::ErrorKind::UserQuit);
                            }
                            _ => (),
                        }
                    } else {
                        let chan = &*self.ui.current_buf()?.to_owned();
                        self.irc_server.send_privmsg(chan, input.get_content())?;

                        let nick = self.irc_server.config().nickname();
                        self.ui.add_event_to_current_chat_buf(
                            model::Event::message(Some(nick), chan, input.get_content())
                        )?;
                    }
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
