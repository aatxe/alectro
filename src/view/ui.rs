use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::sync::{Arc, Mutex, MutexGuard};

use irc::proto::ChannelExt;

use error;
use model::Event;
use view::Terminal;
use view::widget::{ChatBuf, Input, TabLine};

#[derive(Clone)]
pub struct UI {
    state: Arc<InterfaceState>,
}

impl UI {
    pub fn new() -> error::Result<UI> {
        Ok(UI {
            state: Arc::new(InterfaceState::new()?),
        })
    }

    pub fn terminal(&self) -> error::Result<MutexGuard<Terminal>> {
        self.state.terminal()
    }

    pub fn new_chat_buf(&self, buf_name: &str) -> error::Result<()> {
        self.state.new_chat_buf(buf_name)
    }

    pub fn remove_chat_buf(&self, buf_name: &str) -> error::Result<()> {
        self.state.remove_chat_buf(buf_name)
    }

    pub fn current_buf(&self) -> error::Result<MutexGuard<String>> {
        self.state.current_buf()
    }

    pub fn switch_to(&self, buf_name: &str) -> error::Result<()> {
        self.state.switch_to(buf_name)
    }

    pub fn add_event_to_chat_buf(&self, buf_name: &str, event: Event) -> error::Result<()> {
        self.state.add_event_to_chat_buf(buf_name, event)
    }

    pub fn add_event_to_current_chat_buf(&self, event: Event) -> error::Result<()> {
        self.state.add_event_to_current_chat_buf(event)
    }

    pub fn input(&self) -> error::Result<MutexGuard<Input>> {
        self.state.input()
    }

    pub fn draw_all(&self) -> error::Result<()> {
        self.state.draw_all()
    }
}

struct InterfaceState {
    term: Mutex<Terminal>,
    current_buf: Mutex<String>,
    chat_bufs: Mutex<HashMap<String, ChatBuf>>,
    input: Mutex<Input>,
    tabline: Mutex<TabLine>,
}

impl InterfaceState {
    fn new() -> error::Result<InterfaceState> {
        let term = Terminal::new()?;
        let buffer = {
            let mut buf = term.current_buf().clone();
            buf.reset();
            let new_bound = buf.bound().minus_height(2);
            buf.resize(new_bound);
            buf
        };

        let chat_bufs = {
            let mut map = HashMap::new();
            map.insert("*default*".to_owned(), ChatBuf::from_buffer(buffer.clone()));
            map
        };

        let tabline = {
            let mut tabline = TabLine::from_buffer(&buffer);
            tabline.add_tab("*default*", true);
            tabline
        };

        Ok(InterfaceState {
            term: Mutex::new(term),
            current_buf: Mutex::new("*default*".to_owned()),
            chat_bufs: Mutex::new(chat_bufs),
            input: Mutex::new(Input::from_buffer(&buffer)),
            tabline: Mutex::new(tabline),
        })
    }

    fn terminal(&self) -> error::Result<MutexGuard<Terminal>> {
        self.term.lock().map_err(|_| error::ErrorKind::LockPoisoned("UI::Terminal").into())
    }

    fn new_chat_buf(&self, buf_name: &str) -> error::Result<()> {
        let mut chat_bufs = self.chat_bufs.lock().map_err(|_| {
            let e: error::Error = error::ErrorKind::LockPoisoned("UI::ChatBufs").into();
            e
        })?;
        let mut tabline = self.tabline.lock().map_err(|_| {
            let e: error::Error = error::ErrorKind::LockPoisoned("UI::TabLine").into();
            e
        })?;
        let mut new_buf = chat_bufs["*default*"].clone();
        new_buf.reset();
        chat_bufs.insert(buf_name.to_owned(), new_buf);
        tabline.add_tab(buf_name, false);
        Ok(())
    }

    fn remove_chat_buf(&self, buf_name: &str) -> error::Result<()> {
        let mut chat_bufs = self.chat_bufs.lock().map_err(|_| {
            let e: error::Error = error::ErrorKind::LockPoisoned("UI::ChatBufs").into();
            e
        })?;
        let _ = chat_bufs.remove(buf_name);
        let mut tabline = self.tabline.lock().map_err(|_| {
            let e: error::Error = error::ErrorKind::LockPoisoned("UI::TabLine").into();
            e
        })?;
        tabline.remove_tab(buf_name)?;
        Ok(())
    }

    fn current_buf(&self) -> error::Result<MutexGuard<String>> {
        self.current_buf.lock().map_err(|_| {
            let e: error::Error = error::ErrorKind::LockPoisoned("UI::CurrentBuf").into();
            e
        })
    }

    fn switch_to(&self, buf_name: &str) -> error::Result<()> {
        let mut current_buf = self.current_buf()?;
        *current_buf = buf_name.to_owned();
        let mut tabline = self.tabline.lock().map_err(|_| {
            let e: error::Error = error::ErrorKind::LockPoisoned("UI::TabLine").into();
            e
        })?;
        tabline.switch_to(buf_name)?;
        Ok(())
    }

    fn add_event_to_chat_buf(&self, buf_name: &str, event: Event) -> error::Result<()> {
        if buf_name.is_channel_name() {
            self.chat_bufs.lock().map_err(|_| {
                let e: error::Error = error::ErrorKind::LockPoisoned("UI::ChatBufs").into();
                e
            })?.get_mut(buf_name).ok_or_else(|| {
                error::ErrorKind::ChannelNotFound(buf_name.to_owned()).into()
            }).map(|buf| buf.push_event(&event))
        } else {
            self.chat_bufs.lock().map_err(|_| {
                let e: error::Error = error::ErrorKind::LockPoisoned("UI::ChatBufs").into();
                e
            })?.get_mut("*default*").ok_or_else(|| {
                error::ErrorKind::ChannelNotFound("*default*".to_owned()).into()
            }).map(|buf| buf.push_event(&event))
        }
    }

    fn add_event_to_current_chat_buf(&self, event: Event) -> error::Result<()> {
        let current_buf = self.current_buf.lock().map_err(|_| {
            let e: error::Error = error::ErrorKind::LockPoisoned("UI::CurrentBuf").into();
            e
        })?;
        self.add_event_to_chat_buf(&*current_buf, event)
    }

    fn input(&self) -> error::Result<MutexGuard<Input>> {
        self.input.lock().map_err(|_| error::ErrorKind::LockPoisoned("UI::Input").into())
    }

    fn draw_all(&self) -> error::Result<()> {
        let mut term = self.terminal()?;
        let current_buf = self.current_buf()?;
        let chat_bufs = self.chat_bufs.lock().map_err(|_| {
            let e: error::Error = error::ErrorKind::LockPoisoned("UI::ChatBufs").into();
            e
        })?;
        let tabline = self.tabline.lock().map_err(|_| {
            let e: error::Error = error::ErrorKind::LockPoisoned("UI::TabLine").into();
            e
        })?;
        let input = self.input()?;

        term.render(chat_bufs.get(&*current_buf).ok_or_else(|| {
            let e: error::Error = error::ErrorKind::ChannelNotFound(current_buf.clone()).into();
            e
        })?);
        term.render(&*tabline);
        term.render(&*input);
        term.draw()?;
        input.draw_cursor()?;
        io::stdout().flush()?;

        Ok(())
    }
}
