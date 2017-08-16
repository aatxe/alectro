use irc::client::prelude::*;

use error;
use model::Event;
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
            &Command::PRIVMSG(ref chan, ref msg) => {
                self.ui.add_event_to_chat_buf(
                    chan, Event::message(message.source_nickname(), chan, msg)
                )?
            }
            &Command::NOTICE(ref chan, ref msg) => {
                self.ui.add_event_to_chat_buf(
                    chan, Event::notice(message.source_nickname(), chan, msg)
                )?
            }
            &Command::JOIN(ref chan, _, _) => {
                self.ui.add_event_to_chat_buf(
                    chan, Event::joined(message.source_nickname(), chan)
                )?
            }
            &Command::PART(ref chan, _) => {
                self.ui.add_event_to_chat_buf(
                    chan, Event::parted(message.source_nickname(), chan)
                )?
            }
            _ => (),
        }

        Ok(())
    }
}
