use fancy_regex::Regex;
use std::fs;

/**
 * Track the current x,y coordinate. Parse the line to determine movement
 * up, down, left or right and record the new positions. Store positions
 * in the rust equivalent of a HashSet to get the final count of
 * unique positions. Positions are a tuple consisting of two i32s.
 */
fn main() {
    let res1 = part1("input.txt");
    println!("Result 1 is {}", res1);
    let res2 = part2("input.txt");
    println!("Result 2 is {}", res2);
}

fn part1(filepath: &str) -> i32 {
    let mut result = 0;
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");
    for l in contents.lines() {
        if process_line1(l) {
            result += 1;
        };
    }
    return result;
}

fn part2(filepath: &str) -> i32 {
    let mut result = 0;
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");
    for l in contents.lines() {
        if process_line2(l) {
            result += 1;
        };
    }
    return result;
}

fn process_line1(line: &str) -> bool {
    let re1 = Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
    let re2 = Regex::new(r"([a-z])\1").unwrap();
    let re3 = Regex::new(r"(ab|cd|pq|xy)").unwrap();
    if re1.is_match(line).unwrap() && re2.is_match(line).unwrap() && !re3.is_match(line).unwrap() {
        println!("{} matches", line);
        return true;
    }
    return false;
}

fn process_line2(line: &str) -> bool {
    let re1 = Regex::new(r"([a-z][a-z]).*\1").unwrap();
    let re2 = Regex::new(r"([a-z])[a-z]\1").unwrap();
    let p1 = re1.is_match(line).unwrap();
    let p2 = re2.is_match(line).unwrap();
    if p1 && p2 {
        println!("{} matches", line);
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    #[test]
    fn test1() {
        let res = part1("test1.txt");
        assert_eq!(2, res);
    }

    #[test]
    fn test2() {
        let res = part2("test2.txt");
        assert_eq!(2, res);
    }
}