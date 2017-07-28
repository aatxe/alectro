error_chain! {
    links {
        Irc(::irc::error::Error, ::irc::error::ErrorKind);
    }

    foreign_links {
        SendKey(::futures::sync::mpsc::SendError<::termion::event::Event>);
    }

    errors {
        ThreadJoinErr(e: String) {
            description("Attempted to join on panicked thread.")
            display("Attempted to join on panicked thread. Thread panicked with:\n{}", e)
        }
    }
}
