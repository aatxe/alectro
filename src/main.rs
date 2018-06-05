extern crate alectro;
extern crate futures;
extern crate irc;
extern crate termion;

use std::env;

use alectro::controller::{InputController, IrcController};
use alectro::input::AsyncKeyInput;
use alectro::view::UI;
use irc::client::prelude::*;

fn main() {
    let ui = UI::new().unwrap();
    let mut reactor = IrcReactor::new().unwrap();

    let default_cfg = Config {
        nickname: Some(format!("aatxe")),
        server: Some(format!("irc.mozilla.org")),
        use_ssl: Some(true),
        .. Default::default()
    };

    let cfg = match env::home_dir() {
        Some(mut path) => {
            path.push(".alectro");
            path.set_extension("toml");
            Config::load(path).unwrap_or(default_cfg)
        },
        None => default_cfg,
    };

    for chan in &cfg.channels() {
        ui.new_chat_buf(chan).unwrap();
    }

    let irc_client = reactor.prepare_client_and_connect(&cfg).unwrap();
    irc_client.identify().unwrap();

    let irc_controller = IrcController::new(ui.clone());
    reactor.register_client_with_handler(irc_client.clone(), move |_, message| {
        irc_controller.handle_message(message)?;
        irc_controller.ui().draw_all()?;
        Ok(())
    });

    let input_controller = InputController::new(irc_client, ui);
    let input_rx = AsyncKeyInput::new();
    reactor.register_future(input_rx.for_each(move |event| {
        input_controller.handle_event(event)?;
        input_controller.ui().draw_all()?;
        Ok(())
    }).map_err(|e| e.into()));

    reactor.run().unwrap();
}
