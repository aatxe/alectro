use std::cmp;

/// A bound representing a rectangular space for rendering in the terminal.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Bound {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Bound {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Bound {
        Bound {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }

    pub fn area(&self) -> u16 {
        self.width * self.height
    }

    pub fn left_border(&self) -> u16 {
        self.x
    }

    pub fn right_border(&self) -> u16 {
        self.x + self.width
    }

    pub fn top_border(&self) -> u16 {
        self.y
    }

    pub fn bottom_border(&self) -> u16 {
        self.y + self.height
    }

    pub fn minus_width(&self, amount: u16) -> Bound {
        Bound {
            x: self.x,
            y: self.y,
            width: self.width - amount,
            height: self.height,
        }
    }

    pub fn minus_height(&self, amount: u16) -> Bound {
        Bound {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height - amount,
        }
    }

    pub fn union(&self, other: &Bound) -> Bound {
        let x1 = cmp::min(self.x, other.x);
        let y1 = cmp::min(self.y, other.y);
        let x2 = cmp::max(self.x + self.width, other.x + other.width);
        let y2 = cmp::max(self.y + self.height, other.y + other.height);

        Bound {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        }
    }
}
