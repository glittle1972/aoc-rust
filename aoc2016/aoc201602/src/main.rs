use std::fs;

mod keypad;
mod direction;

use direction::Direction;
use keypad::Keypad;

fn main() {
    let result = part1("     \n 123 \n 456 \n 789 \n     ", 2, 2, "input.txt");
    println!("Result is {result:?}");
    let result2= part1("       \n   1   \n  234  \n 56789 \n  ABC  \n   D   \n       ", 1, 3, "input.txt");
    println!("Result is {result2:?}");
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

fn process_line(keypad: &mut Keypad, line: &Vec<Direction>) -> char {
    for direction in line {
        keypad.move_direction(direction);
    }
    keypad.get_value()
}

fn part1(layout: &str, x: usize, y: usize, filepath: &str) -> Vec<char> {
    let mut keypad = Keypad::new(layout, x, y);
    
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
        let expected: Vec<char> = vec!['1', '9', '8', '5'];
        let result= part1("     \n 123 \n 456 \n 789 \n     ", 2, 2, "test.txt");
        assert_eq!(expected, result);
    }

    #[test]
    fn test2() {
        let expected: Vec<char> = vec!['5', 'D', 'B', '3'];
        let result= part1("       \n   1   \n  234  \n 56789 \n  ABC  \n   D   \n       ", 1, 3, "test.txt");
        assert_eq!(expected, result);
    }

}
