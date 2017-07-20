extern crate futures;
extern crate irc;
extern crate termion;
extern crate tokio_core;

pub mod view;

use view::View;

use std::io;
use std::io::Write;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use futures::sync::mpsc;
use irc::client::prelude::*;
use termion::event::Key;
use termion::input::TermRead;
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

    let (keys_tx, keys_rx) = mpsc::unbounded();
    let _ = thread::spawn(move || {
        let stdin = io::stdin();
        let mut input = stdin.keys();

        for res_key in input.next() {
            if let Ok(key) = res_key {
                keys_tx.send(key).unwrap();
            }
        }
    });

    let mut stdout = io::stdout();
    let view = Arc::new(View::new().unwrap());
    let draw_view = view.clone();

    let _ = thread::spawn(move || {
        loop {
            draw_view.draw().unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    let output_future = irc_server.stream().for_each(|message| {
        view.push(message);
        Ok(())
    });

    let input_future = keys_rx.map_err(|()| unreachable!()).for_each(|key| {
        match key {
            Key::Ctrl('c') => {
                irc_server.send_quit("QUIT")?;
            }
            _ => (),
        }

        Ok(())
    });

    reactor.run(output_future.join(input_future)).unwrap();

    write!(stdout, "{}{}{}",
        termion::clear::All, termion::style::Reset, termion::cursor::Goto(1, 1)
    ).unwrap();
    stdout.flush().unwrap();
}
