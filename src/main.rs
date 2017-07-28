extern crate alectro;
extern crate futures;
extern crate irc;
extern crate termion;
extern crate tokio_core;

use std::thread;
use std::time::Duration;

use alectro::controller::InputController;
use alectro::input::AsyncKeyInput;
use alectro::view::UI;
use irc::client::prelude::*;
use tokio_core::reactor::Core;

fn main() {
    let mut reactor = Core::new().unwrap();

    let cfg = Config {
        nickname: Some(format!("aatxe")),
        server: Some(format!("irc.fyrechat.net")),
        channels: Some(vec![format!("#irc-crate")]),
        .. Default::default()
    };

    let irc_server = IrcServer::from_config(cfg).unwrap();
    irc_server.identify().unwrap();

    let ui = UI::new().unwrap();
    let draw_ui = ui.clone();

    let _ = thread::spawn(move || {
        loop {
            draw_ui.draw_all().unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    let output_future = irc_server.stream().map_err(|e| e.into()).for_each(|message| {
        match &message.command {
            &Command::PRIVMSG(_, ref msg)
            | &Command::NOTICE(_, ref msg) => {
                ui.chat_buf().unwrap().push_line(
                    &match message.source_nickname() {
                        Some(nick) => format!("{}: {}", nick, msg),
                        None => format!("{}", msg),
                    }
                )
            }
            &Command::JOIN(ref chan, _, _) => {
                ui.chat_buf().unwrap().push_line(
                    &format!("{} joined {}.", message.source_nickname().unwrap_or("DEFAULT"), chan)
                )
            }
            &Command::PART(ref chan, _) => {
                ui.chat_buf().unwrap().push_line(
                    &format!("{} left {}.", message.source_nickname().unwrap_or("DEFAULT"), chan)
                )
            }
            _ => (),
        }
        Ok(())
    });

    let input_controller = InputController::new(irc_server, ui.clone());
    let input_rx = AsyncKeyInput::new();
    let input_future = input_rx.for_each(|event| input_controller.handle_event(event));

    reactor.run(output_future.join(input_future)).unwrap();
}
