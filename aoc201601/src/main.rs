
mod position;
mod vector;
mod line;

use std::fs;

use position::Position;
use line::Line;

/**
 * OK, for part 2, we need to do vector intersections. At least it's simplified
 * up/down,left/right only. Create a vector struct with add an intersects and
 * getEnd() methods.
 * 
 */
enum Turn { Left, Right, None }
struct Step {
    turn: Turn,
    dist: i16
}

fn main() {
    let result = part1("input.txt");
    println!("Result is {result}");
}

fn turn(heading: i16, turn: Turn) -> i16 {
    let heading = match turn {
        Turn::Left => heading - 90,
        Turn::Right => heading + 90,
        _ => heading
    };
    (heading + 360) % 360
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

fn part1(filepath: &str) ->  i16 {
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");
    
    let mut lines = vec![];
    
    let mut steps = vec![];
    parse_line(contents, &mut steps);
    
    let mut heading = 0;
    let mut pos = Position::new(0, 0);
    
    for step in steps {
        heading = turn(heading, step.turn);
        let line = Line::new_from_heading(pos, heading, step.dist);
        pos = line.get_end();
        lines.push(line);
    }

    pos.get_x().abs() + pos.get_y().abs()
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
