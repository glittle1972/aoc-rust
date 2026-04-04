use crate::direction::Direction;
pub struct Keypad {
    position: i8
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad { position: 5}
    }

    pub fn move_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => {
                let new = self.position - 3;
                if new > 0 {
                    self.position = new;
                }
            },
            Direction::Down => {
                let new = self.position + 3;
                if new < 10 {
                    self.position = new;
                }
            },
            Direction::Right => {
                if self.position % 3 != 0 {
                    self.position += 1;
                }
            },
            Direction::Left => {
                if self.position % 3 != 1 {
                    self.position -= 1;
                }
            }
        }
    }

    pub fn get_value(&self) -> i8 {
        self.position
    }
}