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
            &Command::PRIVMSG(_, ref msg) => {
                self.ui.chat_buf()?.push_line(
                    &match message.source_nickname() {
                        Some(nick) => format!("{}: {}", nick, msg),
                        None => format!("{}", msg),
                    },
                    None,
                )
            }
            &Command::NOTICE(_, ref msg) => {
                self.ui.chat_buf()?.push_line(
                    &match message.source_nickname() {
                        Some(nick) => format!("{}: {}", nick, msg),
                        None => format!("{}", msg),
                    },
                    Some(Style {
                        fg: Color::LightWhite,
                        bg: Color::Yellow,
                        ..Style::default()
                    }),
                )
            }
            &Command::JOIN(ref chan, _, _) => {
                self.ui.chat_buf()?.push_line(
                    &format!("{} joined {}.", message.source_nickname().unwrap_or("DEFAULT"), chan),
                    Some(Color::LightBlack.into()),
                )
            }
            &Command::PART(ref chan, _) => {
                self.ui.chat_buf()?.push_line(
                    &format!("{} left {}.", message.source_nickname().unwrap_or("DEFAULT"), chan),
                    Some(Color::LightBlack.into()),
                )
            }
            _ => (),
        }

        Ok(())
    }
}
