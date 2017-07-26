use view::{Buffer};

mod chatbuf;

pub use self::chatbuf::ChatBuf;

pub trait Widget {
    fn draw(&self, buffer: &mut Buffer);
}
