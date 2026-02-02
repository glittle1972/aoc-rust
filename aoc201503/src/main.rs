use std::{collections::HashSet, fs};

/**
 * Track the current x,y coordinate. Parse the line to determine movement
 * up, down, left or right and record the new positions. Store positions
 * in the rust equivalent of a HashSet to get the final count of
 * unique positions. Positions are a tuple consisting of two i32s.
 */
fn main() {
    let res1 = part1("input.txt");
    for r in res1 {
        println!("Result is {}", r);
    }
    let res2 = part2("input.txt");
    for r in res2 {
        println!("Result is {}", r);
    }
}

fn part1(filepath: &str) -> [i32; 3] {
    let mut result = [0, 0, 0];
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");
    let mut i = 0;
    for l in contents.lines() {
        result[i] = process_line1(l); i += 1;
    }
    return result;
}

fn part2(filepath: &str) -> [i32; 3] {
    let mut result = [0, 0, 0];
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");
    let mut i = 0;
    for l in contents.lines() {
        result[i] = process_line2(l); i += 1;
    }
    return result;
}

fn process_line1(line: &str) -> i32 {
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

fn process_line2(line: &str) -> i32 {
    let mut visited_points: HashSet<(i32, i32)> = HashSet::new();
    let mut pos1 = (0, 0);
    let mut pos2 = (0, 0);
    visited_points.insert(pos1);
    let mut even = true;
    for c in line.chars() {
        if even {
            process_move(c, &mut pos1);
            visited_points.insert(pos1);
            even = !even;
        } else {
            process_move(c, &mut pos2);
            visited_points.insert(pos2);
            even = !even;
        }
    }
    return visited_points.len().try_into().unwrap();
}

fn process_move(c: char, pos: &mut (i32, i32)) {
    match c {
        '^' => pos.1 = pos.1 + 1,
        '>' => pos.0 = pos.0 + 1,
        'v' => pos.1 = pos.1 - 1,
        '<' => pos.0 = pos.0 - 1,
        _ => eprintln!("Unexpected character, {}", c),
    }    
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    #[test]
    fn test1() {
        let res = part1("test1.txt");
        assert_eq!([2,4,2], res);
    }

   #[test]
    fn test2() {
        let res = part2("test2.txt");
        assert_eq!([3,3,11], res);
    }
}