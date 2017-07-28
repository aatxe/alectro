use irc::client::prelude::*;

use error;
use view::UI;

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
            &Command::PRIVMSG(_, ref msg) |
            &Command::NOTICE(_, ref msg) => {
                self.ui.chat_buf()?.push_line(
                    &match message.source_nickname() {
                        Some(nick) => format!("{}: {}", nick, msg),
                        None => format!("{}", msg),
                    }
                )
            }
            &Command::JOIN(ref chan, _, _) => {
                self.ui.chat_buf()?.push_line(
                    &format!("{} joined {}.", message.source_nickname().unwrap_or("DEFAULT"), chan)
                )
            }
            &Command::PART(ref chan, _) => {
                self.ui.chat_buf()?.push_line(
                    &format!("{} left {}.", message.source_nickname().unwrap_or("DEFAULT"), chan)
                )
            }
            _ => (),
        }

        Ok(())
    }
}
