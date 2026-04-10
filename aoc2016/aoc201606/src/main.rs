use std::collections::HashMap;
use std::fs;

fn main() {
    let result = part1("input.txt");
    println!("Result is {result:?}");
}

fn parse_contents(contents: String, strings: &mut Vec<String>) {
    for line in contents.lines() {
        strings.push(String::from(line));
    }
}

/**
 * Return the most common character at index position from the list
 * of strings.
 */
fn get_most_common(strings: &Vec<String>, index: usize) -> char {
    let mut counts = HashMap::new();
    for string in strings {
        let c = match string.chars().nth(index) {
            Some(c) => c,
            None => ' '
        };
        *counts.entry(c).or_insert(0) += 1;
    }
    *counts.iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k).unwrap()
}

fn part1(filepath: &str) -> String {
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");
    
    let mut strings = vec![];
    
    parse_contents(contents, &mut strings);
    
    let length = match strings.first() {
        Some(first) => first.len(),
        None => 0
    };
    
    let mut result = vec![];
    for index in 0..length {
        result.push(get_most_common(&strings, index));
    }
    
    result.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test1() {
        static EXPECTED: &str = "easter";
        let result= part1("test.txt");
        assert_eq!(EXPECTED, result);
    }

}
