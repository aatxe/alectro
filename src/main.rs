extern crate irc;
extern crate termion;

use std::io::prelude::*;
use std::io;
use std::sync::{Arc, mpsc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use irc::client::prelude::*;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let terminate = Arc::new(AtomicBool::new(false));
    let terminate_stdin = terminate.clone();

    let cfg = Config {
        nickname: Some(format!("aatxe")),
        server: Some(format!("irc.fyrechat.net")),
        use_ssl: Some(true),
        channels: Some(vec![format!("#irc-crate")]),
        .. Default::default()
    };

    let irc_server = IrcServer::from_config(cfg).unwrap();
    irc_server.identify().unwrap();

    let (keys_tx, keys_rx) = mpsc::channel();
    let _ = thread::spawn(move || {
        let terminate = terminate_stdin;
        let stdin = io::stdin();
        let mut input = stdin.keys();

        while let Some(res_key) = input.next() {
            if terminate.load(Ordering::Relaxed) {
                break
            }

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

    let mut stream_iter = irc_server.stream().wait();
    while let Some(res) = stream_iter.next() {
        write!(stdout, "{}", res.unwrap()).unwrap();

        if let Ok(Key::Char('q')) = keys_rx.try_recv() {
            irc_server.send_quit("User sent a quit command.").unwrap();
            break;
        }
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
    stdout.flush().unwrap();
}
