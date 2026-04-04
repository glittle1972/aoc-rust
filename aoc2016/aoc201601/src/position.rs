use std::fmt;

pub struct Position {
    x: i16,
    y: i16
}

impl Position {
    pub fn new(x: i16, y: i16) -> Position {
        Self { x, y }
    }

    pub fn get_x(&self) -> i16 {
        self.x
    }

    pub fn get_y(&self) -> i16 {
        self.y
    }

    pub fn get_block_distance(&self) -> i16 {
        self.x.abs() + self.y.abs()
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "position: ({}, {})", self.x, self.y)
    }
}