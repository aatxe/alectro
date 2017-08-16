use irc::client::prelude::*;

use error;
use utils;
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
                        Some(nick) => format!(
                            "<{}{}{}> {}", utils::colorize(nick).to_irc_color(), nick,
                            Color::Reset.to_irc_color(), msg
                        ),
                        None => format!("{}", msg),
                    },
                    None,
                )?
            }
            &Command::NOTICE(ref chan, ref msg) => {
                self.ui.add_line_to_chat_buf(
                    chan,
                    &match message.source_nickname() {
                        Some(nick) => format!(
                            "<{}{}{}{}{}> {}", utils::colorize(nick).to_irc_color(), "\x02", nick,
                            "\x02", format!("\x03{},{}", Color::LightWhite, Color::Yellow), msg
                        ),
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
                let nick = message.source_nickname().unwrap_or("DEFAULT");
                self.ui.add_line_to_chat_buf(
                    chan,
                    &format!(
                        "{}{}{} joined {}.", utils::colorize(nick).to_irc_color(), nick,
                        Color::LightBlack.to_irc_color(), chan
                    ),
                    Some(Color::LightBlack.into()),
                )?
            }
            &Command::PART(ref chan, _) => {
                let nick = message.source_nickname().unwrap_or("DEFAULT");
                self.ui.add_line_to_chat_buf(
                    &chan[..],
                    &format!(
                        "{}{}{} left {}.", utils::colorize(nick).to_irc_color(), nick,
                        Color::LightBlack.to_irc_color(), chan),
                    Some(Color::LightBlack.into()),
                )?
            }
            _ => (),
        }

        Ok(())
    }
}
