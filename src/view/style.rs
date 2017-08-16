use std::fmt;

use termion;
use termion::color::{Fg, Bg};

macro_rules! make_color {
    ($(#[$attr:meta] $variant:ident = $value:expr),+) => {
        #[derive(Debug, Clone, Copy, Eq, PartialEq)]
        #[repr(u8)]
        pub enum Color {
            $(#[$attr] $variant = $value),+
        }

        impl fmt::Display for Color {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", match self {
                    $(&Color::$variant => $value),+
                })
            }
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

            pub fn to_irc_color(self) -> String {
                match self {
                    Color::Reset => "\x03".to_owned(),
                    _ => format!("\x03{}", self),
                }
            }

            pub fn from_u8(val: u8) -> Option<Color> {
                match val {
                    $($value => Some(Color::$variant),)+
                    _ => None
                }
            }
        }
    }
}

make_color! {
    /// Black.
    Black = 1,
    /// Blue.
    Blue = 2,
    /// Cyan.
    Cyan = 10,
    /// Green.
    Green = 3,
    /// High-intensity light black.
    LightBlack = 14,
    /// High-intensity light blue.
    LightBlue = 12,
    /// High-intensity light cyan.
    LightCyan = 11,
    /// High-intensity light green.
    LightGreen = 9,
    /// High-intensity light magenta.
    LightMagenta = 13,
    /// High-intensity light red.
    LightRed = 4,
    /// High-intensity light white.
    LightWhite = 0,
    /// High-intensity light yellow.
    LightYellow = 8,
    /// Magenta.
    Magenta = 6,
    /// Red.
    Red = 5,
    /// Reset colors to default.
    Reset = 255,
    /// White.
    White = 15,
    /// Yellow.
    Yellow = 7
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

impl Modifier {
    pub fn inverted(&self) -> Option<Modifier> {
        use self::Modifier::*;

        Some(match *self {
            Blink => NoBlink,
            Bold => NoBold,
            CrossedOut => NoCrossedOut,
            Faint => NoFaint,
            Invert => NoInvert,
            Italic => NoItalic,
            NoBlink => Blink,
            NoBold => Bold,
            NoCrossedOut => CrossedOut,
            NoFaint => Faint,
            NoInvert => Invert,
            NoItalic => Italic,
            NoUnderline => Underline,
            Underline => NoUnderline,
            Framed | Reset => return None,
        })
    }
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

    pub fn modifier_with_toggle(mut self, modifier: Modifier) -> Style {
        if self.modifier == modifier {
            if let Some(inverted) = modifier.inverted() {
                // FIXME: We want to use inverted here, but most terminals don't support it. To get
                // around this, we will need to add support in the renderer for translating inverted
                // modifiers into a reset followed by the restoration of any additional modifiers.
                self.modifier = Modifier::Reset;
            } else {
                self.modifier = Modifier::Reset;
            }
        } else {
            self.modifier = modifier;
        }
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
