use std::fs;

mod keypad;
mod direction;

use direction::Direction;
use keypad::Keypad;

fn main() {
    let result = part1("input.txt");
    println!("Result is {result:?}");
}

fn parse_lines(contents: String, lines: &mut Vec<Vec<Direction>>) {
    for line in contents.lines() {
        let mut directions = vec![];
        for c in line.chars() {
            match c {
                'U' => directions.push(Direction::Up),
                'D' => directions.push(Direction::Down),
                'L' => directions.push(Direction::Left),
                'R' => directions.push(Direction::Right),
                _ =>  ()
            }
        }
        lines.push(directions);
    }
}

fn process_line(keypad: &mut Keypad, line: &Vec<Direction>) -> i8 {
    for direction in line {
        keypad.move_direction(direction);
    }
    keypad.get_value()
}

fn part1(filepath: &str) -> Vec<i8> {
    let mut keypad = Keypad::new();
    
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");
    
    let mut lines = vec![];
    
    parse_lines(contents, &mut lines);
    
    let mut result = vec![];

    for line in lines {
        result.push(process_line(&mut keypad, &line));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test1() {
        let expected: Vec<i8> = vec![1, 9, 8, 5];
        let result= part1("test.txt");
        assert_eq!(expected, result);
    }

}
