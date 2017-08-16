use utils;
use view::{Color, Style};

pub enum Event {
    /// sender, target, message
    PrivMessage(Option<String>, String, String),
    /// sender, target, message
    Notice(Option<String>, String, String),
    /// sender, target, joined
    JoinPart(Option<String>, String, bool),
}

impl Event {
    pub fn message(sender: Option<&str>, target: &str, message: &str) -> Event {
        Event::PrivMessage(sender.map(|s| s.to_owned()), target.to_owned(), message.to_owned())
    }

    pub fn notice(sender: Option<&str>, target: &str, message: &str) -> Event {
        Event::Notice(sender.map(|s| s.to_owned()), target.to_owned(), message.to_owned())
    }

    pub fn joined(sender: Option<&str>, target: &str) -> Event {
        Event::JoinPart(sender.map(|s| s.to_owned()), target.to_owned(), true)
    }

    pub fn parted(sender: Option<&str>, target: &str) -> Event {
        Event::JoinPart(sender.map(|s| s.to_owned()), target.to_owned(), false)
    }

    pub fn style(&self) -> Option<Style> {
        None
    }
}

impl ToString for Event {
    fn to_string(&self) -> String {
        match self {
            &Event::PrivMessage(ref sender, _, ref message) => {
                let nick = sender.as_ref().map(|s| &s[..]).unwrap_or("");
                format!(
                    "\x03{}[{}{}\x03{}]{} {}", Color::Magenta, utils::colorize(nick).to_irc_color(),
                    nick, Color::Magenta, Color::Reset.to_irc_color(), message
                )
            }
            &Event::Notice(Some(ref sender), _, ref message) => {
                format!(
                    "\x03{}*[{}\x02{}\x02\x03{}]*{} {}", Color::Magenta,
                    utils::colorize(sender).to_irc_color(), sender, Color::Magenta,
                    Color::Reset.to_irc_color(), message
                )
            }
            &Event::Notice(None, _, ref message) => {
                format!("\x03{}*{} {}", Color::Magenta, Color::Reset.to_irc_color(), message)
            }
            &Event::JoinPart(Some(ref sender), _, true) => {
                format!(
                    "\x03{}+{}{}{}", Color::Green, utils::colorize(sender).to_irc_color(), sender,
                    Color::Reset.to_irc_color()
                )
            }
            &Event::JoinPart(Some(ref sender), _, false) => {
                format!(
                    "\x03{}-{}{}{}", Color::Red, utils::colorize(sender).to_irc_color(), sender,
                    Color::Reset.to_irc_color()
                )
            }
            &Event::JoinPart(None, _, _) => "".to_owned(),
        }
    }
}
