extern crate alectro;
extern crate futures;
extern crate irc;
extern crate termion;
extern crate tokio_core;

use std::io;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use alectro::view::Terminal;
use alectro::view::widget::ChatBuf;
use futures::sync::mpsc;
use irc::client::prelude::*;
use termion::event::Key;
use termion::input::TermRead;
use tokio_core::reactor::Core;

fn main() {
    let mut term = Terminal::new().unwrap();
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

    let buffer = {
        let mut buf = term.current_buf().clone();
        buf.reset();
        buf
    };
    let chat_buf = Arc::new(Mutex::new(ChatBuf::from_buffer(buffer)));
    let draw_chat_buf = chat_buf.clone();

    let _ = thread::spawn(move || {
        loop {
            term.render(
                draw_chat_buf.lock().unwrap().deref()
            );
            term.draw().unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    let output_future = irc_server.stream().for_each(|message| {
        match &message.command {
            &Command::PRIVMSG(_, ref msg)
            | &Command::NOTICE(_, ref msg) => {
                chat_buf.lock().unwrap().push_line(
                    &match message.source_nickname() {
                        Some(nick) => format!("{}: {}", nick, msg),
                        None => format!("{}", msg),
                    }
                )
            }
            &Command::JOIN(ref chan, _, _) => {
                chat_buf.lock().unwrap().push_line(
                    &format!("{} joined {}.", message.source_nickname().unwrap_or("DEFAULT"), chan)
                )
            }
            &Command::PART(ref chan, _) => {
                chat_buf.lock().unwrap().push_line(
                    &format!("{} left {}.", message.source_nickname().unwrap_or("DEFAULT"), chan)
                )
            }
            _ => (),
        }
        Ok(())
    });

    let input_future = keys_rx.map_err(|()| unreachable!()).for_each(|key| {
        match key {
            Key::Ctrl('c') | Key::Ctrl('d') | Key::Ctrl('q') => {
                irc_server.send_quit("QUIT")?;
            }
            _ => (),
        }

        Ok(())
    });

    reactor.run(output_future.join(input_future)).unwrap();
}
