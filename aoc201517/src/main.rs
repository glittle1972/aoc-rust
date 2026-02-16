use std::fs;
use std::time::Instant;

fn main() {
    let result1 = part1("input.txt", 150);
    println!("result1 is {}", result1);
}

fn part1(filepath: &str, amount: usize) -> usize {
    let start = Instant::now();

    let mut bottles: Vec<usize> = vec![];
    
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");    
    parse_lines(contents, &mut bottles);
    dbg!(&bottles);

    const BASE: usize = 2;
    let num_bottles = bottles.len();
    let num_combs = BASE.pow(num_bottles as u32) - 1;

    let mut count = 0;
    for c in 0..num_combs {
        let mut total = 0;
        for i in 0..num_bottles {
            if BASE.pow(i as u32) & c != 0 {
                total += bottles[i];
            }
        }
        if total == amount {
            count += 1;
        }
    }

    let dur = start.elapsed();
    println!("Duration = {:?}", dur);

    return count;
}

fn parse_lines(lines: String, bottles: &mut Vec<usize>) {
    for line in lines.lines() {
        bottles.push(line.parse::<usize>().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test1() {
        let result1= part1("test.txt", 25);
        assert_eq!(4, result1);
    }

}
