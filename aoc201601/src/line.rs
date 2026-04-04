use crate::position::Position;
use crate::vector::Vector;

pub struct Line {
    start: Position,
    dir: Vector
}

impl Line {
    pub fn new(start: Position, dir: Vector) -> Line {
        Self { start, dir }
    }

    pub fn new_from_heading(start: Position, heading: i16, dist: i16) -> Line {
        let dir = Vector::new_from_heading(heading, dist);
        Line::new(start, dir)
    }

    pub fn get_end(&self) -> Position {
        Position::new(self.start.get_x() + self.dir.get_x(), 
            self.start.get_y() + self.dir.get_y())
    }
}