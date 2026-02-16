use std::fs;
use std::time::Instant;

fn main() {
    let (total, min_total) = part1("input.txt", 150);
    println!("result1 is {}", total);
    println!("result2 is {}", min_total);
}

fn part1(filepath: &str, amount: usize) -> (usize, usize) {
    let start = Instant::now();

    let mut bottles: Vec<usize> = vec![];
    
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");    
    parse_lines(contents, &mut bottles);
    dbg!(&bottles);

    const BASE: usize = 2;
    let num_bottles = bottles.len();
    let num_combos = BASE.pow(num_bottles as u32) - 1;

    let mut count: Vec<usize> = vec![];
    for c in 0..num_combos {
        let mut total = 0;
        let mut num = 0;
        for i in 0..num_bottles {
            if BASE.pow(i as u32) & c != 0 {
                total += bottles[i];
                num += 1;
            }
        }
        if total == amount {
            count.push(num);
        }
    }

    // length of count is the total number of combos that meet the total
    // filter by minimum to get the num of combos using the min.
    let total = count.len();
    let min = count.iter().min().unwrap();
    let min_total = count.iter().filter(|v| *v == min).count();

    let dur = start.elapsed();
    println!("Duration = {:?}", dur);

    return (total, min_total);
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
        let (total, min_total) = part1("test.txt", 25);
        assert_eq!(4, total);
        assert_eq!(3, min_total);
    }

}
