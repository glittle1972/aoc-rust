use std::fs;

fn main() {
    let result = part1("input.txt");
    println!("Result is {result:?}");
}

fn part1(filepath: &str) -> usize {    
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");
    
    let mut valid: usize = 0;

    let lines = contents.lines();
    for line in lines {
        let mut v: Vec<usize> = line.trim().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
        v.sort();
        if v[0] + v[1] > v[2] {
            valid += 1;
        }
    }

    valid
}
