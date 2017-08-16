use view::Color;

pub fn colorize(name: &str) -> Color {
    let value = name.bytes().enumerate().map(|(i, c)| i as u16 * c as u16).sum::<u16>() % 12;
    Color::from_u8(value as u8 + 2).unwrap()
}
