use std::cmp::min;
use std::collections::{HashMap,HashSet};
use std::fs;

use itertools::Itertools;

/**
 * AJust brute force it using itertool::permutations
 */

fn main() {
    let res1 = part1("input.txt");
    println!("Result 1 is {}", res1);
}

fn part1(filepath: &str) -> u16 {
    let mut edges = HashMap::new();
    let mut vertices = HashSet::new();

    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");
    
    for l in contents.lines() {
        let (v1, v2, d) = parse_line(l);
        edges.insert(v1.to_owned() + v2.as_str(), d);
        edges.insert(v2.to_owned() + v1.as_str(), d);
        vertices.insert(v1);
        vertices.insert(v2);
    }
    dbg!(&edges);

    let mut result = std::u16::MAX;

    // Create all permutations of verticies. This is going to be n!
    // This could probably be halved as the graph is undirected but let's
    // go easiest first
    let perms = vertices.iter().permutations(vertices.len());
    for perm in perms.clone() {
        let d = get_distance(&edges, perm);
        result = min(result, d);
        println!("Current shortest is {}", result);
    }
    return result;
}

fn get_distance(edges: &HashMap<String, u16>, path: Vec<&String>) -> u16 {
    let mut result = 0;

    for i in 0..path.len() - 1 {
        let v1 = path[i];
        let v2 = path[i+1];
        let lookup = v1.to_owned() + v2.as_str();
        let r = edges.get(&lookup).unwrap();
        result += r;
    }

    println!("{:?} -> {}", path, result);
    return result;
}

fn parse_line(line: &str) -> (String, String, u16) {
    let tokens = line.split(" ").collect::<Vec<&str>>();
    
    let v1 = tokens[0].to_string();
    let v2 = tokens[2].to_string();
    return (v1, v2, tokens[4].parse::<u16>().unwrap());
}


#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test1() {
        let res = part1("test.txt");
        assert_eq!(605, res);
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