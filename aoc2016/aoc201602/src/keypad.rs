use crate::direction::Direction;
pub struct Keypad {
    layout: Vec<Vec<char>>,
    x: usize,
    y: usize
}

impl Keypad {
    pub fn new(layout: &str, x:usize, y: usize) -> Keypad {
        let mut grid = vec![];
        let lines = layout.lines();
        for line in lines {
            let mut row = vec![];
            for c in line.chars() {
                row.push(c);
            }
            grid.push(row);
        }
        Keypad { layout: grid, x, y }
    }

    pub fn move_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => {
                let ny = self.y - 1;
                if self.layout[ny][self.x] != ' ' {
                    self.y = ny;
                }
            },
            Direction::Down => {
                let ny = self.y + 1;
                if self.layout[ny][self.x] != ' ' {
                    self.y = ny;
                }
            },
            Direction::Right => {
                let nx = self.x + 1;
                if self.layout[self.y][nx] != ' ' {
                    self.x = nx;
                }
            },
            Direction::Left => {
                let nx = self.x - 1;
                if self.layout[self.y][nx] != ' ' {
                    self.x = nx;
                }
            }
        }
    }

    pub fn get_value(&self) -> char {
        self.layout[self.y][self.x]
    }
}