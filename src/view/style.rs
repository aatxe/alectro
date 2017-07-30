use termion;
use termion::color::{Fg, Bg};

macro_rules! make_color {
    ($(#[$attr:meta] $variant:ident),+) => {
        #[derive(Debug, Clone, Copy, Eq, PartialEq)]
        pub enum Color {
            $(#[$attr] $variant),+
        }

        impl Color {
            pub fn to_fg_string(self) -> String {
                match self {
                    $(Color::$variant => format!("{}", Fg(termion::color::$variant))),+
                }
            }

            pub fn to_bg_string(self) -> String {
                match self {
                    $(Color::$variant => format!("{}", Bg(termion::color::$variant))),+
                }
            }
        }
    }
}

make_color! {
    /// Black.
    Black,
    /// Blue.
    Blue,
    /// Cyan.
    Cyan,
    /// Green.
    Green,
    /// High-intensity light black.
    LightBlack,
    /// High-intensity light blue.
    LightBlue,
    /// High-intensity light cyan.
    LightCyan,
    /// High-intensity light green.
    LightGreen,
    /// High-intensity light magenta.
    LightMagenta,
    /// High-intensity light red.
    LightRed,
    /// High-intensity light white.
    LightWhite,
    /// High-intensity light yellow.
    LightYellow,
    /// Magenta.
    Magenta,
    /// Red.
    Red,
    /// Reset colors to default.
    Reset,
    /// White.
    White,
    /// Yellow.
    Yellow
}

macro_rules! make_modifier {
    ($(#[$attr:meta] $variant:ident),+) => {
        #[derive(Debug, Clone, Copy, Eq, PartialEq)]
        pub enum Modifier {
            $(#[$attr] $variant),+
        }

        impl Modifier {
            pub fn to_string(self) -> String {
                match self {
                    $(Modifier::$variant => format!("{}", termion::style::$variant)),+
                }
            }
        }
    }
}

make_modifier! {
    /// Blinking text (not widely supported).
    Blink,
    /// Bold text.
    Bold,
    /// Crossed out text (not widely supported).
    CrossedOut,
    /// Fainted text (not widely supported).
    Faint,
    /// Framed text (not widely supported).
    Framed,
    /// Inverted colors (negative mode).
    Invert,
    /// Italic text.
    Italic,
    /// Undo blinking text (not widely supported).
    NoBlink,
    /// Undo bold text.
    NoBold,
    /// Undo crossed out text (not widely supported).
    NoCrossedOut,
    /// Undo fainted text (not widely supported).
    NoFaint,
    /// Undo inverted colors (negative mode).
    NoInvert,
    /// Undo italic text.
    NoItalic,
    /// Undo underlined text.
    NoUnderline,
    /// Reset SGR parameters.
    Reset,
    /// Underlined text.
    Underline
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub modifier: Modifier,
}

impl Style {
    pub fn fg(mut self, color: Color) -> Style {
        self.fg = color;
        self
    }

    pub fn bg(mut self, color: Color) -> Style {
        self.bg = color;
        self
    }

    pub fn modifier(mut self, modifier: Modifier) -> Style {
        self.modifier = modifier;
        self
    }

    pub fn reset(&mut self) {
        self.fg = Color::Reset;
        self.bg = Color::Reset;
        self.modifier = Modifier::Reset;
    }

}

impl Default for Style {
    fn default() -> Style {
        Style {
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: Modifier::Reset,
        }
    }
}

impl From<Color> for Style {
    fn from(color: Color) -> Style {
        Style {
            fg: color,
            ..Style::default()
        }
    }
}

impl From<Modifier> for Style {
    fn from(modifier: Modifier) -> Style {
        Style {
            modifier: modifier,
            ..Style::default()
        }
    }
}
