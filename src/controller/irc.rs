use irc::client::data::User;
use irc::client::prelude::*;
use irc::proto::ChannelExt;

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
                if !chan.is_channel_name() || self.ui.has_chat_buf(chan)? {
                    self.ui.add_event_to_chat_buf(
                        chan, Event::message(message.source_nickname(), chan, msg)
                    )?
                }
            }
            &Command::NOTICE(ref chan, ref msg) => {
                if !chan.is_channel_name() || self.ui.has_chat_buf(chan)? {
                    self.ui.add_event_to_chat_buf(
                        chan, Event::notice(message.source_nickname(), chan, msg)
                    )?
                }
            }
            &Command::JOIN(ref chan, _, _) => {
                if !chan.is_channel_name() || self.ui.has_chat_buf(chan)? {
                    self.ui.add_event_to_chat_buf(
                        chan, Event::joined(message.source_nickname(), chan)
                    )?
                }
            }
            &Command::PART(ref chan, _) => {
                if !chan.is_channel_name() || self.ui.has_chat_buf(chan)? {
                    self.ui.add_event_to_chat_buf(
                        chan, Event::parted(message.source_nickname(), chan)
                    )?
                }
            }
            &Command::Response(Response::RPL_NAMREPLY, ref args, ref suffix) => {
                if let Some(chan) = args.iter().find(|s| s.is_channel_name()) {
                    if let Some(users) = suffix.as_ref().map(|s| s.split(" ")) {
                        for user in users {
                            // Skip empty strings.
                            if user.is_empty() {
                                continue;
                            }

                            // Parses off the access level information.
                            let user = User::new(user);
                            let nickname = user.get_nickname();
                            self.ui.add_event_to_chat_buf(
                                chan, Event::joined(Some(nickname), chan)
                            )?
                        }
                    }
                }
            }
            _ => (),
        }

        Ok(())
    }
}
