use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref re_open: Regex = Regex::new(r#"(^\")"#).unwrap();
    static ref re_close: Regex = Regex::new(r#"(\"$)"#).unwrap();
    static ref re_backslash: Regex = Regex::new(r#"(\\\\)"#).unwrap();
    static ref re_quote: Regex = Regex::new(r#"(\\\")"#).unwrap();
    static ref re_hex: Regex = Regex::new(r#"(\\x[0-9a-f]{2})"#).unwrap();
}

fn main() {
    let res1 = part1("input.txt");
    println!("Result 1 is {}", res1);
    let res2 = part2("input.txt");
    println!("Result 2 is {}", res2);
}

fn part1(filepath: &str) -> usize {
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");
    
    let mut result = 0;
    for l in contents.lines() {
        let (total, memory) = process_line(l);
        result += total - memory;
    }

    return result;
}

fn part2(filepath: &str) -> usize {
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");
    
    let mut result = 0;
    for l in contents.lines() {
        let (total, memory) = process_line2(l);
        result += memory - total;
    }

    return result;
}

fn process_line(line: &str) -> (usize, usize) {
    let total = line.bytes().len();
//    let caps = re_line.captures(line).unwrap();
    println!("total for {} is {}", line, total);
    let s1 = re_open.replace_all(line, "");
    let s2 = re_close.replace_all(&s1, "");
    let s3 = re_backslash.replace_all(&s2, "B");
    let s4 = re_quote.replace_all(&s3, "Q");
    let s5 = re_hex.replace_all(&s4, "X");
    let memory = s5.bytes().len();
    println!("total for {} is {}", s5, memory);

    return (total, memory);
}

fn process_line2(line: &str) -> (usize, usize) {
    let total = line.bytes().len();
//    let caps = re_line.captures(line).unwrap();
    println!("total for {} is {}", line, total);
    let s1 = re_backslash.replace_all(&line, "BBBB");
    let s2 = re_hex.replace_all(&s1, "BBxXX");
    let s3 = re_quote.replace_all(&s2, "BBBQ");
    let s4 = re_open.replace_all(&s3, "QBQ");
    let s5 = re_close.replace_all(&s4, "BQQ");
    let memory = s5.bytes().len();
    println!("total for {} is {}", s5, memory);

    return (total, memory);
}


#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    #[test]
    fn test1() {
        let res = part1("test.txt");
        assert_eq!(12, res);
    }

    #[test]
    fn test2() {
        let res = part2("test.txt");
        assert_eq!(19, res);
    }

}