use std::io;
use std::io::Write;
use std::sync::{Arc, Mutex, MutexGuard};

use error;
use view::Terminal;
use view::widget::{ChatBuf, Input};

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

    pub fn chat_buf(&self) -> error::Result<MutexGuard<ChatBuf>> {
        self.state.chat_buf()
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
    chat_buf: Mutex<ChatBuf>,
    input: Mutex<Input>,
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

        Ok(InterfaceState {
            term: Mutex::new(term),
            chat_buf: Mutex::new(ChatBuf::from_buffer(buffer.clone())),
            input: Mutex::new(Input::from_buffer(buffer)),
        })
    }

    fn terminal(&self) -> error::Result<MutexGuard<Terminal>> {
        self.term.lock().map_err(|_| error::ErrorKind::LockPoisoned("UI::Terminal").into())
    }

    fn chat_buf(&self) -> error::Result<MutexGuard<ChatBuf>> {
        self.chat_buf.lock().map_err(|_| error::ErrorKind::LockPoisoned("UI::ChatBuf").into())
    }

    fn input(&self) -> error::Result<MutexGuard<Input>> {
        self.input.lock().map_err(|_| error::ErrorKind::LockPoisoned("UI::Input").into())
    }

    fn draw_all(&self) -> error::Result<()> {
        let mut term = self.terminal()?;
        let chat_buf = self.chat_buf()?;
        let input = self.input()?;

        term.render(&*chat_buf);
        term.render(&*input);
        term.draw()?;
        input.draw_cursor()?;
        io::stdout().flush()?;

        Ok(())
    }
}
