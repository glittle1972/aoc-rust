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
}