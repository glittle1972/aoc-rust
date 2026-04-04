use std::cmp::max;
use std::collections::{HashMap,HashSet};
use std::fs;
use std::time::Instant;

use itertools::Itertools;
use regex::Regex;

/**
 * APPROACH: Each pairing can be resolved into a single +ve or -ve
 * happiness change. Let's store each of those in a HashMap.
 * Then a simple depth first search tracking the largest path.
 * When we get to the bottom, we need to add on the happiness
 * index for the last vertex back to the first. (Is this TSP?)
 * 
 * In a cache, we can store totals for any longer sequence of vertices,
 * e.g. DEFG -> 23
 */
fn main() {
    let result1 = part1("input.txt");
    println!("result1 is {}", result1);
    let result2 = part1("input2.txt");
    println!("result2 is {}", result2);
}

fn part1(filepath: &str) -> i32 {
    let start = Instant::now();

    let mut edges: HashMap<String, i32> = HashMap::new();
    let mut vertices: HashSet<String> = HashSet::new();

    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");
    
    parse_lines(contents, &mut edges, &mut vertices);

    let mut mx = std::i32::MIN;

    // Create all permutations of vertices. This is going to be n!
    // This could probably be halved as the graph is undirected but let's
    // go easiest first
    let perms = vertices.iter().permutations(vertices.len());
    for mut perm in perms.clone() {
        perm.push(perm[0]);
        let d = get_happiness(&edges, perm);
        mx = max(mx, d);
        println!("Current happiness is {}", mx);
    }

    let dur = start.elapsed();
    println!("Duration = {:?}", dur);

    return mx;
}

fn parse_lines(lines: String, edges: &mut HashMap<String, i32>, vertices: &mut HashSet<String>) {
    let re = Regex::new(r"(?<v1>[A-Z][a-z]*) would (?<op>[a-z]+) (?<val>[0-9]+) happiness units by sitting next to (?<v2>[A-Z][a-z]+)").unwrap();
    for line in lines.lines() {
        let caps = re.captures(line).unwrap();
        let v1 = String::from(&caps["v1"][..1]);
        let op = &caps["op"];
        let val = match op {
            "gain" => caps["val"].parse::<i32>().unwrap(),
            "lose" => 0 - caps["val"].parse::<i32>().unwrap(),
            _ => 0,
        };
        let v2 = String::from(&caps["v2"][..1]);
        let key = format!("{}{}", v1, v2);
        let rev = format!("{}{}", v2, v1);
        vertices.insert(v1);
        vertices.insert(v2);
        if edges.contains_key(&key) {
            let og = *edges.get(&key).unwrap();
            edges.insert(key, og + val);
            edges.insert(rev, og + val);
        } else {
            edges.insert(key, val);
            edges.insert(rev, val);
        }
    }
}

fn get_happiness(edges: &HashMap<String, i32>, path: Vec<&String>) -> i32 {
    let mut result = 0;

    println!("Testing {:?}", path);
    for i in 0..path.len() - 1 {
        let v1 = path[i];
        let v2 = path[i+1];
        let lookup = format!("{}{}", v1, v2);
        let r = edges.get(&lookup).unwrap();
        result += r;
    }

    println!("{:?} -> {}", path, result);
    return result;
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test1() {
        let result1= part1("test.txt");
        assert_eq!(330, result1);
    }

}

#[cfg(test)]
mod tests2 {
    use itertools::Itertools;

    #[test]
    fn test1() {
        let input = vec!["alpha", "bravo", "charlie", "delta"];
        let perms = input.iter().permutations(4);
        for p in perms {
            dbg!(p);
        }
    }
}