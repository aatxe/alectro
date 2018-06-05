use std::io::Error as IoError;

use futures::sync::mpsc::SendError;
use irc::error::IrcError;
use termion::event::Event;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "an io error occurred")]
    Io(#[cause] IoError),

    #[fail(display = "failed to send keypress event")]
    SendKey(#[cause] SendError<Event>),

    #[fail(display = "irc error")]
    Irc(#[cause] IrcError),

    #[fail(display = "attempted to join on panicked thread. thread panicked with:\n{}", err)]
    ThreadJoinErr {
        err: String,
    },

    #[fail(display = "failed to acquire poisoned lock: {}", lock)]
    LockPoisoned {
        lock: &'static str,
    },

    #[fail(display = "failed to look up the specified channel: {}", chan)]
    ChannelNotFound {
        chan: String,
    },

    #[fail(display = "failed to find the specified tab: {}", tab)]
    TabNotFound {
        tab: String,
    },

    #[fail(display = "the user initiated a quit command")]
    UserQuit,
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Error {
        Error::Io(e)
    }
}

impl From<SendError<Event>> for Error {
    fn from(e: SendError<Event>) -> Error {
        Error::SendKey(e)
    }
}

impl From<IrcError> for Error {
    fn from(e: IrcError) -> Error {
        Error::Irc(e)
    }
}

impl From<Error> for IrcError {
    fn from(error: Error) -> IrcError {
        match error {
            Error::Irc(e) => e,
            _ => IrcError::Custom { inner: error.into() },
        }
    }
}
