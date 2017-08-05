extern crate alectro;
extern crate futures;
extern crate irc;
extern crate termion;
extern crate tokio_core;

use alectro::controller::{InputController, IrcController};
use alectro::input::AsyncKeyInput;
use alectro::view::UI;
use irc::client::prelude::*;
use tokio_core::reactor::Core;

fn main() {
    let ui = UI::new().unwrap();
    let mut reactor = Core::new().unwrap();

    let cfg = Config {
        nickname: Some(format!("alectro")),
        server: Some(format!("irc.pdgn.co")),
        channels: Some(vec![format!("#pdgn"), format!("#ctf")]),
        use_ssl: Some(true),
        .. Default::default()
    };

    for chan in &cfg.channels() {
        ui.new_chat_buf(chan).unwrap();
    }

    let irc_server = IrcServer::from_config(cfg).unwrap();
    irc_server.identify().unwrap();

    let irc_controller = IrcController::new(ui.clone());
    let output_future = irc_server.stream().map_err(|e| e.into()).for_each(|message| {
        irc_controller.handle_message(message)?;
        irc_controller.ui().draw_all()
    });

    let input_controller = InputController::new(irc_server, ui);
    let input_rx = AsyncKeyInput::new();
    let input_future = input_rx.for_each(|event| {
        input_controller.handle_event(event)?;
        input_controller.ui().draw_all()
    });

    reactor.run(output_future.join(input_future)).unwrap();
}
