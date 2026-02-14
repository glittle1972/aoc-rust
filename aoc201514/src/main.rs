use std::cmp::{min,max};
use std::fs;
use std::time::Instant;

use regex::Regex;

#[derive(Debug)]
struct Reindeer {
    name: String,
    vel: u32,
    dur: u32,
    rest: u32,
    distance: u32,
    score: u32
}

fn main() {
    let result1 = part1("input.txt", 2503);
    println!("result1 is {}", result1);
    let result2 = part2("input.txt", 2503);
    println!("result2 is {}", result2);
}

fn part1(filepath: &str, time: u32) -> u32 {
    let start = Instant::now();

    let mut reindeer: Vec<Reindeer> = vec![];
    
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");    
    parse_lines(contents, &mut reindeer);

    let mut furthest = std::u32::MIN;
    for mut r in reindeer {
        calc_distance(time, &mut r);
        furthest = max(furthest, r.distance);
        println!("Current furthest is {}", furthest);
    }

    let dur = start.elapsed();
    println!("Duration = {:?}", dur);

    return furthest;
}

fn part2(filepath: &str, time: u32) -> u32 {
    let start = Instant::now();

    let mut reindeer: Vec<Reindeer> = vec![];
    
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");    
    parse_lines(contents, &mut reindeer);

    // Like part1, except we have to iterate up to the total time and get the distance
    // for each reindeer at each time.
    let mut furthest = std::u32::MIN;
    for t in 0..time {
        for mut r in &mut reindeer {
            calc_distance(t + 1, &mut r);
            furthest = max(furthest, r.distance);
        }
        for r in &mut reindeer {
            if r.distance == furthest {
                r.score += 1;
            }
        }
        println!("{:?}", reindeer);
    }

    let dur = start.elapsed();
    println!("Duration = {:?}", dur);

    return reindeer.iter().map(|r| r.score).max().unwrap();
}

fn parse_lines(lines: String, reindeer: &mut Vec<Reindeer>) {
    // Dancer can fly 37 km/s for 1 seconds, but then must rest for 36 seconds.
    let re = Regex::new(r"(?<name>[A-Z][a-z]+) can fly (?<vel>[0-9]+) km/s for (?<dur>[0-9]+) seconds, but then must rest for (?<rest>[0-9]+) seconds.").unwrap();
    for line in lines.lines() {
        let caps = re.captures(line).unwrap();
        let name = String::from(&caps["name"]);
        let vel = caps["vel"].parse::<u32>().unwrap();
        let dur = caps["dur"].parse::<u32>().unwrap();
        let rest = caps["rest"].parse::<u32>().unwrap();
        reindeer.push(Reindeer { name, vel, dur, rest, distance: 0, score: 0 });
    }
}

/**
 * Right, some modulo arithmetic to get how many cycles go and stop,
 * then work out any moving time left.
 */
fn calc_distance(time: u32, r: &mut Reindeer) {

    // in a full cycle we complete vel * dur
    let full = time / (r.dur + r.rest);
    r.distance = r.vel * r.dur * full;
    
    // calc remaining seconds (should be less than dur + rest)
    let rem = time - full * (r.dur + r.rest);
    // we're only interested in moving time now, so it max dur
    let mov = min(rem, r.dur);
    r.distance += r.vel * mov;

    println!("Distance for {} is {}", r.name, r.distance);
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    #[test]
    fn test1() {
        let result1= part1("test.txt", 1000);
        assert_eq!(1120, result1);
    }

    #[test]
    fn test2() {
        let result2= part2("test.txt", 1000);
        assert_eq!(689, result2);
    }

}
