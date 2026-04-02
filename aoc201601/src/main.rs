
use std::fs;

enum Turn { Left, Right, None }
enum Direction { North, South, East, West }
struct Step {
    turn: Turn,
    dist: i16
}
struct Position {
    x: i16,
    y: i16
}

fn main() {
    let result = part1("input.txt");
    println!("Result is {result}");
}

fn turn(current: &Direction, turn: Turn) -> Direction {
    match current {
        Direction::North => {
            match turn {
                Turn::Left => Direction::West,
                Turn::Right => Direction::East,
                _ => Direction::North
            }
        },
        Direction::South => {
            match turn {
                Turn::Left => Direction::East,
                Turn::Right => Direction::West,
                _ => Direction::South
            }
        },
        Direction::East => {
            match turn {
                Turn::Left => Direction::North,
                Turn::Right => Direction::South,
                _ => Direction::East
            }
        },
        _ => {
            match turn {
                Turn::Left => Direction::South,
                Turn::Right => Direction::North,
                _ => Direction::West
            }
        }
    }
}

fn travel(current: &Direction, dist: i16, pos: Position) -> Position {
    match current {
        Direction::North => Position { x: pos.x, y: pos.y + dist },
        Direction::South => Position { x: pos.x, y: pos.y - dist },
        Direction::East => Position { x: pos.x + dist, y: pos.y },
        _ => Position { x: pos.x - dist, y: pos.y }
    }
}

fn parse_step(step: &str) -> Step {
    let turn = &step[..1];
    let turn = match turn {
        "L" => Turn::Left,
        "R" => Turn::Right,
        _ => Turn::None
    };
    let dist = &step[1..];
    let dist = dist.parse::<i16>().unwrap();
    Step { turn, dist }
}

fn parse_line(line: String, steps: &mut Vec<Step>) {
    for step in line.split(", ") {
        let step = parse_step(step);
        steps.push(step);
    }
}

fn part1(filepath: &str) -> i16 {
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");
    
    let mut steps = vec![];
    parse_line(contents, &mut steps);
    let mut current = Direction::North;
    let mut pos = Position { x: 0, y: 0 };
    
    for step in steps {
        current = turn(&current, step.turn);
        pos = travel(&current, step.dist, pos);
    }

    pos.x.abs() + pos.y.abs()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::part1;

    #[test]
    fn test1() {
        let mut tests = HashMap::new();
        tests.insert("test1.txt", 5);
        tests.insert("test2.txt", 2);
        tests.insert("test3.txt", 12);
        for (_filepath, expected) in tests {
            let result= part1(_filepath);
            assert_eq!(expected, result);
        }
    }
}
