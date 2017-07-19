extern crate futures;
extern crate irc;
extern crate termion;
extern crate tokio_core;

use std::io::prelude::*;
use std::io;
use std::thread;

use futures::sync::mpsc;
use irc::client::prelude::*;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tokio_core::reactor::Core;

fn main() {
    let mut reactor = Core::new().unwrap();

    let cfg = Config {
        nickname: Some(format!("aatxe")),
        server: Some(format!("irc.fyrechat.net")),
        use_ssl: Some(true),
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

    let mut stdout = io::stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide).unwrap();

    let output_future = irc_server.stream().for_each(|message| {
        print!("{}", message);
        Ok(())
    });

    let input_future = keys_rx.map_err(|()| unreachable!()).for_each(|key| {
        match key {
            Key::Char('q') => {
                irc_server.send_quit("QUIT")?;
            }
            _ => (),
        }

        Ok(())
    });

    reactor.run(output_future.join(input_future)).unwrap();

    write!(stdout, "{}", termion::cursor::Show).unwrap();
    stdout.flush().unwrap();
}
