use regex::Regex;
use std::fs;

/**
 * Track the current x,y coordinate. Parse the line to determine movement
 * up, down, left or right and record the new positions. Store positions
 * in the rust equivalent of a HashSet to get the final count of
 * unique positions. Positions are a tuple consisting of two i32s.
 */
fn main() {
    let res1 = part1("input.txt");
    println!("Result is {}", res1);
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

fn process_line1(line: &str) -> bool {
    let re1 = Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
    let re2 = Regex::new(r"(aa|bb|cc|dd|ee|ff|gg|hh|ii|jj|kk|ll|mm|nn|oo|pp|qq|rr|ss|tt|uu|vv|ww|xx|yy|zz)").unwrap();
    let re3 = Regex::new(r"(ab|cd|pq|xy)").unwrap();
    if re1.is_match(line) && re2.is_match(line) && !re3.is_match(line) {
        println!("{} matches", line);
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test1() {
        let res = part1("test1.txt");
        assert_eq!(2, res);
    }
}