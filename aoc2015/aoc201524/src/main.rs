use std::cmp::min;
use std::fs;
use std::time::Instant;

use itertools::Itertools;

fn parse_lines(lines: String, weights: &mut Vec<u32>) {
    for line in lines.lines() {
        weights.push(line.parse::<u32>().unwrap());
    }
}

fn part1(filepath: &str, groups: u32) -> u64 {
    let start = Instant::now();

    let mut weights = vec![];

    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");    
    parse_lines(contents, &mut weights);

    let total_weight = weights.iter().sum::<u32>();
    let group_weight = total_weight / groups;
    weights.sort_by(|a,b| b.cmp(a));

    let mut minimum = std::u64::MAX;

    // We only care about the first group.
    // Look for permutations of 1 package that weigh group_weight
    // then perms of 2, perms of 3...
    // Get the quantum entanglement of each and compare to min.
    // The max number of packages in the lest group is total number
    // of packages divided by the number of groups.
    for size in 1..(weights.len() / 3) + 1 {
        let perms = weights.iter().permutations(size);
        for perm in perms {
            if perm.iter().map(|r| *r).sum::<u32>() == group_weight {
                println!("Checking {:?}", perm);
                let qe = perm.iter().map(|r| (**r) as u64).product::<u64>();
                println!("Quantum Entaglement = {}", qe);
                minimum = min(minimum, qe);
                println!("Minimum = {}", minimum);
            }
        }
        // If we find a solution for size, then we're only interested
        // in this minimum - having additional packages can't win.
        if minimum < std::u64::MAX {
            break;
        }
    }

    let dur = start.elapsed();
    println!("Duration = {:?}", dur);

    return minimum;
}

fn main() {
    let result1 = part1("input.txt", 3);
    println!("result1 is {}", result1);
    let result2 = part1("input.txt", 4);
    println!("result2 is {}", result2);
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test1() {
        let result1= part1("test.txt", 3);
        assert_eq!(99, result1);
    }

    #[test]
    fn test2() {
        let result1= part1("test.txt", 4);
        assert_eq!(44, result1);
    }

}
