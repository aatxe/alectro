use view::{Buffer};

mod chatbuf;
mod input;
mod tabline;

pub use self::chatbuf::ChatBuf;
pub use self::input::Input;
pub use self::tabline::TabLine;

pub trait Widget {
    fn draw(&self, buffer: &mut Buffer);
}
