use std::cmp::{min,max};
use std::fmt;

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

    pub fn find_intersection(&self, other: &Line) -> Option<Position> {
        print!("Testing {self} against {other} ");

        let self_x_min = min(self.start.get_x(), self.get_end().get_x());
        let self_x_max = max(self.start.get_x(), self.get_end().get_x());
        let self_y_min = min(self.start.get_y(), self.get_end().get_y());
        let self_y_max = max(self.start.get_y(), self.get_end().get_y());
        let other_x_min = min(other.start.get_x(), other.get_end().get_x());
        let other_x_max = max(other.start.get_x(), other.get_end().get_x());
        let other_y_min = min(other.start.get_y(), other.get_end().get_y());
        let other_y_max = max(other.start.get_y(), other.get_end().get_y());
        // exclude parallel lines
        let int = if (self_x_max - self_x_min == 0 && other_x_max - other_x_min == 0 ) ||
            (self_y_max - self_y_min == 0 && other_y_max - other_y_min == 0 ) {
            None
        } else 
        // self is vertical and other is horizontal
        if self_x_min >= other_x_min && self_x_min <= other_x_max &&
            other_y_min >= self_y_min && other_y_min <= self_y_max {
            Some(Position::new(self_x_min, other_y_min))
        } else
            // self is horizontal and other is vertical
        if self_y_min >= other_y_min && self_y_min <= other_y_max &&
            other_x_min >= self_x_min && other_x_min <= self_x_max {
            Some(Position::new(other_x_min, self_y_min))
        } else {
            None
        };

        match &int {
            Some(int) => println!("{}", int),
            None => println!("None")
        };
        
        int
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line: ({}, {}) -> ({}, {})", self.start.get_x(), self.start.get_y(),
            self.dir.get_x(), self.dir.get_y())
    }
}