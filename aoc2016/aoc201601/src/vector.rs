pub struct Vector {
    x: i16,
    y: i16
}

impl Vector {
    pub fn new(x: i16, y: i16) -> Vector {
        Self{ x, y }
    }

    pub fn new_from_heading(heading: i16, dist: i16) -> Vector {
        match heading {
            0 => Self::new(0, dist),
            90 => Self::new(dist, 0),
            180 => Self::new(0, 0 - dist),
            _ => Self::new(0 - dist, 0)
        }
    }

    pub fn get_x(&self) -> i16 {
        self.x
    }

    pub fn get_y(&self) -> i16 {
        self.y
    }
}
