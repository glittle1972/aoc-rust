use std::fs;

use regex::Regex;

fn main() {
    let results = part1("input.txt");
    println!("Results are {:?}", results);
}

fn part1(filepath: &str) -> Vec<i32> {

    let reg = Regex::new(r"(-{0,1}[0-9]+)").unwrap();

    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");
    
    let mut results: Vec<i32> = Vec::new();
    for l in contents.lines() {
        let sum = reg.find_iter(l).map(|m| m.as_str().parse::<i32>().unwrap()).sum();
        results.push(sum);
    }

    return results;
}


#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test1() {
        let results = part1("test1.txt");
        let expected: Vec<i32> = vec![6, 6, 3, 3, 0, 0, 0, 0];
        assert_eq!(expected, results);
    }

}
