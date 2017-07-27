use view::{Buffer};

mod chatbuf;
mod input;

pub use self::chatbuf::ChatBuf;
pub use self::input::Input;

pub trait Widget {
    fn draw(&self, buffer: &mut Buffer);
}
