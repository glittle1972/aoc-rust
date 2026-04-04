
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
    println!("Result is {result:?}");
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

// Technically, the end of the previous line, and the start of the
// current intersect, but we don;t want to count this. So, start
// from the third line, and stop at c - 2.
fn find_first_insersection(lines: &Vec<Line>) -> Option<Position> {
    let size = lines.len();
    for cur in 2..size {
        for prev in 0..cur - 1 {
            let cur = lines.get(cur).unwrap();
            let prev = lines.get(prev).unwrap();
            match cur.find_intersection(prev) {
                Some(intersection) => return Some(intersection),
                None => ()
            }
        }
    }
    None
}

fn part1(filepath: &str) ->  (i16, i16) {
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

    let intersection = match find_first_insersection(&lines) {
        Some(intersection) => intersection,
        None => Position::new(0, 0)
    };

    (pos.get_block_distance(), intersection.get_block_distance())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::part1;

    #[test]
    fn test1() {
        let mut tests = HashMap::new();
        tests.insert("test1.txt", (5, 0));
        tests.insert("test2.txt", (2, 0));
        tests.insert("test3.txt", (12, 0));
        tests.insert("test4.txt", (8, 4));
        for (filepath, expected) in tests {
            let result= part1(filepath);
            assert_eq!(expected, result);
        }
    }

}
