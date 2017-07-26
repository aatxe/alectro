error_chain! {
    foreign_links {
        Irc(::irc::error::Error);
    }
}
