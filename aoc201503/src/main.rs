use std::{collections::HashSet, fs};

/**
 * Track the current x,y coordinate. Parse the line to determine movement
 * up, down, left or right and record the new positions. Store positions
 * in the rust equivalent of a HashSet to get the final count of
 * unique positions. Positions are a tuple consisting of two i32s.
 */
fn main() {
    let res = part1("input.txt");
    for r in res {
        println!("Result is {}", r);
    }
}

fn part1(filepath: &str) -> [i32; 3] {
    let mut result = [0, 0, 0];
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");
    let mut i = 0;
    for l in contents.lines() {
        result[i] = process_line(l); i += 1;
    }
    return result;
}

fn process_line(line: &str) -> i32 {
    let mut visited_points: HashSet<(i32, i32)> = HashSet::new();
    let mut pos = (0, 0);
    visited_points.insert(pos);
    for c in line.chars() {
        match c {
            '^' => pos.1 = pos.1 + 1,
            '>' => pos.0 = pos.0 + 1,
            'v' => pos.1 = pos.1 - 1,
            '<' => pos.0 = pos.0 - 1,
            _ => eprintln!("Unexpected character, {}", c),
        }
        visited_points.insert(pos);
    }
    return visited_points.len().try_into().unwrap();
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test1() {
        let res = part1("test.txt");
        assert_eq!([2,4,2], res);
    }
}