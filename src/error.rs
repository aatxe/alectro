error_chain! {
    links {
        Irc(::irc::error::Error, ::irc::error::ErrorKind);
    }

    foreign_links {
        SendKey(::futures::sync::mpsc::SendError<::termion::event::Event>);
        Io(::std::io::Error);
    }

    errors {
        ThreadJoinErr(e: String) {
            description("Attempted to join on panicked thread.")
            display("Attempted to join on panicked thread. Thread panicked with:\n{}", e)
        }
        LockPoisoned(s: &'static str) {
            description("Failed to acquire lock because it was poisoned.")
            display("Failed to acquire lock {} because it was poisoned.", s)
        }
        UserQuit {
            description("The user requested to quit the program.")
            display("The user requested to quit the program.")
        }
    }
}
