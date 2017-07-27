use std::collections::VecDeque;

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use view::Bound;

/// A single cell in the terminal buffer.
#[derive(Clone, PartialEq, Eq)]
pub struct Cell {
    pub grapheme: String,
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new(" ")
    }
}

impl Cell {
    fn new(grapheme: &str) -> Cell {
        Cell {
            grapheme: grapheme.to_owned(),
        }
    }

    fn reset(&mut self) {
        self.grapheme = " ".to_owned();
    }
}

/// A complete terminal buffer used in rendering.
#[derive(Clone)]
pub struct Buffer {
    bound: Bound,
    buf: VecDeque<Cell>,
}

impl Buffer {
    pub fn empty(bound: Bound) -> Buffer {
        Buffer {
            buf: {
                let size = bound.area() as usize;
                let mut vec = VecDeque::with_capacity(size);
                for _ in 0..size {
                    vec.push_back(Cell::default())
                }
                vec
            },
            bound: bound,
        }
    }

    pub fn bound(&self) -> &Bound {
        &self.bound
    }

    pub fn height(&self) -> u16 {
        self.bound.height
    }

    pub fn width(&self) -> u16 {
        self.bound.width
    }

    pub fn get(&self, x: u16, y: u16) -> &str {
        &self.buf[self.index_of(x, y)].grapheme
    }

    pub fn set(&mut self, x: u16, y: u16, c: &str) {
        let idx = self.index_of(x, y);
        self.buf[idx] = Cell::new(c);
    }

    /// Sets the cells starting at (x, y) to string s without performing wrapping.
    pub fn set_str(&mut self, mut x: u16, y: u16, s: &str) {
        let graphemes = UnicodeSegmentation::graphemes(s, true);

        for g in graphemes {
            self.set(x, y, g);
            x += g.width() as u16;
        }
    }

    pub fn resize(&mut self, bound: Bound) {
        let size = bound.area() as usize;
        if self.buf.len() > size {
            self.buf.truncate(size)
        } else {
            self.buf.resize(size, Cell::default())
        }
        self.bound = bound;
    }

    pub fn drop_top_line(&mut self) {
        for _ in 0..self.bound.width {
            self.buf.pop_front();
            self.buf.push_back(Cell::default());
        }
    }

    pub fn inner(&self) -> &VecDeque<Cell> {
        &self.buf
    }

    /// Resets all cells in the buffer.
    pub fn reset(&mut self) {
        for c in &mut self.buf {
            c.reset();
        }
    }

    /// Merge the given buffer onto this one.
    pub fn merge(&mut self, other: &Buffer) {
        let bound = self.bound.union(&other.bound);

        // Add any additional cells necessary with the default cell value.
        self.buf.resize(bound.area() as usize, Cell::default());

        // Move original buf contents to the appropriate cell.
        let offset_x = self.bound.x - bound.x;
        let offset_y = self.bound.y - bound.y;
        let size = self.bound.area() as usize;
        for i in (0..size).rev() {
            let (x, y) = self.pos_of(i);
            let new_idx = ((y + offset_y) * bound.width + (x + offset_x)) as usize;

            // Move the contents around if necessary.
            if i != new_idx {
                self.buf[i] = Cell::default();
                self.buf.swap(new_idx, i);
            }
        }

        // Push contents of the other buffer into this one, erasing any already present cells.
        let size = other.bound.area() as usize;
        for i in 0..size {
            let (x, y) = other.pos_of(i);
            let new_idx = self.index_of(x, y);
            self.buf[new_idx] = other.buf[i].clone();
        }

        self.bound = bound;
    }

    pub fn pos_of(&self, i: usize) -> (u16, u16) {
        debug_assert!(
            i < self.buf.len(),
            "Attempted to determine coordinates of a position outside the buffer: i={} len={}",
            i,
            self.buf.len()
        );

        (self.bound.x + i as u16 % self.bound.width, self.bound.y + i as u16 / self.bound.width)
    }

    fn index_of(&self, x: u16, y: u16) -> usize {
        debug_assert!(
            x >= self.bound.left_border() && x < self.bound.right_border() &&
                y >= self.bound.top_border() && y < self.bound.bottom_border(),
            "Attempted to access a point outside of the buffer: x={}, y={}, bound={:?}",
            x,
            y,
            self.bound,
        );

        ((y - self.bound.y) * self.bound.width + (x - self.bound.x)) as usize
    }
}
