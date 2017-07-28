extern crate alectro;
extern crate futures;
extern crate irc;
extern crate termion;
extern crate tokio_core;

use std::io;
use std::io::Write;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use alectro::input::AsyncKeyInput;
use alectro::view::Terminal;
use alectro::view::widget::{ChatBuf, Input};
use futures::sync::mpsc;
use irc::client::prelude::*;
use termion::event::{Event, Key};
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

    let buffer = {
        let mut buf = term.current_buf().clone();
        buf.reset();
        let new_bound = buf.bound().minus_height(2);
        buf.resize(new_bound);
        buf
    };
    let chat_buf = Arc::new(Mutex::new(ChatBuf::from_buffer(buffer.clone())));
    let draw_chat_buf = chat_buf.clone();
    let input = Arc::new(Mutex::new(Input::from_buffer(buffer)));
    let draw_input = input.clone();

    let _ = thread::spawn(move || {
        loop {
            term.render(
                draw_chat_buf.lock().unwrap().deref()
            );
            let inpt = draw_input.lock().unwrap();
            term.render(
                inpt.deref()
            );
            term.draw().unwrap();
            inpt.set_cursor().unwrap();
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    let output_future = irc_server.stream().map_err(|e| e.into()).for_each(|message| {
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

    let input_rx = AsyncKeyInput::new();

    let input_future = input_rx.for_each(|event| {
        if let Event::Key(key) = event {
            match key {
                Key::Ctrl('c') | Key::Ctrl('d') | Key::Ctrl('q') => {
                    irc_server.send_quit("QUIT")?;
                    panic!("User quit."); // This is a terrible way to exit.
                }
                Key::Char('\n') => {
                    let mut inpt = input.lock().unwrap();
                    irc_server.send_privmsg("#irc-crate", inpt.get_content())?;
                    chat_buf.lock().unwrap().push_line(
                        &format!("{}: {}", irc_server.config().nickname(), inpt.get_content())
                    );
                    inpt.reset();
                }
                Key::Char(c) => {
                    input.lock().unwrap().add_char(c);
                }
                Key::Backspace => {
                    input.lock().unwrap().backspace();
                }
                Key::Left => {
                    input.lock().unwrap().move_left();
                }
                Key::Right => {
                    input.lock().unwrap().move_right();
                }
                Key::Up => {
                    input.lock().unwrap().move_up();
                }
                Key::Down => {
                    input.lock().unwrap().move_down();
                }
                _ => (),
            }
        }

        Ok(())
    });

    reactor.run(output_future.join(input_future)).unwrap();
}
