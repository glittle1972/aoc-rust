use std::time::Instant;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref re: Regex = Regex::new(r"(1+|2+|3+|4+|5+|6+|7+|8+|9+)").unwrap();
}

fn main() {
    let result = part1("1113222113", 40);
    println!("Result is {} with length {}", result, result.len());
    let start = Instant::now();
    let result2 = part1("1113222113", 50);
    println!("Result 2 is {} with length {}", result2, result2.len());
    let dur = start.elapsed();
    println!("Duration = {:?}", dur);
}

fn part1(input: &str, iter: usize) -> String {
    let mut result = String::from(input);
    for _ in 0..iter {
        result = parse(result);
    }
    return result;
}

fn parse(input: String) -> String {
    let mut result = String::new();
    
    for m in re.find_iter(&input) {
        expand(m.as_str(), &mut result);
    }
    return result;
}

fn expand(expr: &str, result: &mut String) {
    let len = expr.len();
    let num = expr.chars().next().unwrap_or('0');
    use std::fmt::Write;
    let _ = write!(result, "{}{}", len, num);
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::expand;

    #[test]
    fn test1() {
        let result = part1("1", 5);
        assert_eq!("312211", result);
    }

    #[test]
    fn test111() {
        let mut result = String::new();
        expand("111", &mut result);
        assert_eq!("31", result);
    }
}