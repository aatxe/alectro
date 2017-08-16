use std::slice::Iter;

use model::Event;

pub struct ChatBuf {
    events: Vec<Event>,
}

impl<'a> IntoIterator for &'a ChatBuf {
    type Item = &'a Event;
    type IntoIter = Iter<'a, Event>;

    fn into_iter(self) -> Iter<'a, Event> {
        self.events.iter()
    }
}
