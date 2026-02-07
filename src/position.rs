use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        return Self {
            x: x,
            y: y
        };
    }

    pub fn zero() -> Self {
        return Self::new(0, 0);
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "[x: {}, y: {}]", self.x, self.y);
    }
}