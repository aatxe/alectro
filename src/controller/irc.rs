use irc::client::prelude::*;

use error;
use view::{Color, Style, UI};

pub struct IrcController {
    ui: UI,
}

impl IrcController {
    pub fn new(ui: UI) -> IrcController {
        IrcController {
            ui: ui,
        }
    }

    pub fn ui(&self) -> &UI {
        &self.ui
    }

    pub fn handle_message(&self, message: Message) -> error::Result<()> {
        match &message.command {
            &Command::PRIVMSG(ref chan, ref msg) => {
                self.ui.add_line_to_chat_buf(
                    chan,
                    &match message.source_nickname() {
                        Some(nick) => format!("{}: {}", nick, msg),
                        None => format!("{}", msg),
                    },
                    None,
                )?
            }
            &Command::NOTICE(ref chan, ref msg) => {
                self.ui.add_line_to_chat_buf(
                    chan,
                    &match message.source_nickname() {
                        Some(nick) => format!("{}: {}", nick, msg),
                        None => format!("{}", msg),
                    },
                    Some(Style {
                        fg: Color::LightWhite,
                        bg: Color::Yellow,
                        ..Style::default()
                    }),
                )?
            }
            &Command::JOIN(ref chan, _, _) => {
                self.ui.add_line_to_chat_buf(
                    chan,
                    &format!("{} joined {}.", message.source_nickname().unwrap_or("DEFAULT"), chan),
                    Some(Color::LightBlack.into()),
                )?
            }
            &Command::PART(ref chan, _) => {
                self.ui.add_line_to_chat_buf(
                    &chan[..],
                    &format!("{} left {}.", message.source_nickname().unwrap_or("DEFAULT"), chan),
                    Some(Color::LightBlack.into()),
                )?
            }
            _ => (),
        }

        Ok(())
    }
}
