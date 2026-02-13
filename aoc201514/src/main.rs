use std::cmp::{min,max};
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use regex::Regex;

fn main() {
    let result1 = part1("input.txt", 2503);
    println!("result1 is {}", result1);
}

fn part1(filepath: &str, time: u32) -> u32 {
    let start = Instant::now();

    let mut reindeer: HashMap<String, (u32, u32, u32)> = HashMap::new();

    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");    
    parse_lines(contents, &mut reindeer);

    // Create all permutations of vertices. This is going to be n!
    // This could probably be halved as the graph is undirected but let's
    // go easiest first
    let mut furthest = std::u32::MIN;
    for k in reindeer.keys() {
        let r = reindeer.get(k).unwrap();
        let dist = get_distance(time, k, r);
        furthest = max(furthest, dist);
        println!("Current furthest is {}", furthest);
    }

    let dur = start.elapsed();
    println!("Duration = {:?}", dur);

    return furthest;
}

fn parse_lines(lines: String, reindeer: &mut HashMap<String, (u32, u32, u32)>) {
    // Dancer can fly 37 km/s for 1 seconds, but then must rest for 36 seconds.
    let re = Regex::new(r"(?<name>[A-Z][a-z]+) can fly (?<vel>[0-9]+) km/s for (?<dur>[0-9]+) seconds, but then must rest for (?<rest>[0-9]+) seconds.").unwrap();
    for line in lines.lines() {
        let caps = re.captures(line).unwrap();
        let name = String::from(&caps["name"]);
        let vel = caps["vel"].parse::<u32>().unwrap();
        let dur = caps["dur"].parse::<u32>().unwrap();
        let rest = caps["rest"].parse::<u32>().unwrap();
        reindeer.insert(name, (vel, dur, rest));
    }
}

/**
 * Right, some modulo arithmetic to get how many cycles go and stop,
 * then work out any moving time left.
 */
fn get_distance(time: u32, name: &str, reindeer: &(u32, u32, u32)) -> u32 {
    let mut result = 0;

    let vel = reindeer.0;
    let dur = reindeer.1;
    let rest = reindeer.2;
    
    // in a full cycle we complete vel * dur
    let full = time / (dur + rest);
    result += vel * dur * full;
    
    // calc remaining seconds (should be less than dur + rest)
    let rem = time - full * (dur + rest);
    // we're only interested in moving time now, so it max dur
    let mov = min(rem, dur);
    result += vel * mov;

    println!("Distance for {} is {}", name, result);
    return result;
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test1() {
        let result1= part1("test.txt", 1000);
        assert_eq!(1120, result1);
    }

}
