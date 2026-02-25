// use std::cmp::min;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use regex::Regex;

fn main() {
    let molecules = part1("input.txt");
    println!("result1 is {}", molecules);
    let steps = part2("input.txt");
    println!("result2 is {}", steps);
}

fn part1(filepath: &str) -> usize {
    let start = Instant::now();

    let mut replacements = vec![];
    
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");    
    let molecule = parse_lines(contents, &mut replacements);
    
    dbg!(&replacements);
    dbg!(&molecule);
    
    let mut molecules = HashSet::new();

    for replacement in replacements {
        // A replacement may match multiple times, but I need to replace
        // each match one at a time
        let re = Regex::new(&replacement.0).unwrap();
        re.find_iter(&molecule).for_each(|m| {
            let mut temp = molecule.clone();
            temp.replace_range(m.range(), &replacement.1);
            molecules.insert(temp);
        });
    }

    let dur = start.elapsed();
    println!("Duration = {:?}", dur);

    return molecules.len();
}

/**
 * I Think I want to do this in reverse - start with the molecule and replace
 * entries in reverse. Try each replacement in reverse and try to get down to 
 * 'e'
 */
fn part2(filepath: &str) -> usize {
    let start = Instant::now();

    let mut replacements = vec![];
    
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");    
    let molecule = parse_lines(contents, &mut replacements);
    // Sort by descending length of the replacement value
    replacements.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    dbg!(&replacements);
    dbg!(&molecule);
    
    let (mut m, mut steps) = (molecule, 0);
    
    // This works because there are no matches that are substrings of another match,
    // So we can never match "Si" and prevent us from matching the better "CaSi". So
    // we can always just iterate over all patterns trying them one by one. It doesn't
    // matter what order we hit the matches in. (This means I didn't need to sort them,
    // above)
    // I got this from https://observablehq.com/@jwolondon/advent-of-code-2015-day-19
    // but not sure I'd have worked it out myself.
    while !m.eq("e") {
        for (to, from) in &replacements {
            match m.find(from) {
                Some(i) => {
                    m = format!("{}{}{}", &m[0..i], to, &m[i + from.len()..]);
                    steps += 1;
                },
                _ => ()
            }
        }
    }

    let dur = start.elapsed();
    println!("Duration = {:?}", dur);

    return steps;
}

fn parse_lines(lines: String, replacements: &mut Vec<(String, String)>) -> String {
    let re_repl = Regex::new(r"([A-Za-z]+) => ([A-Za-z]+)").unwrap();
    let re_mol = Regex::new(r"[A-Za-z]+").unwrap();

    let mut molecule: String = "".to_string();

    for line in lines.lines() {
        let caps = re_repl.captures(line);
        if caps.is_some() {
            let cap = caps.unwrap();
            replacements.push((String::from(&cap[1]), String::from(&cap[2])));
        } else {
            let mols = re_mol.captures(line);
            if mols.is_some() {
                let mol = mols.unwrap();
                molecule = String::from(&mol[0]);
            }
        }
    }

    return molecule;
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    #[test]
    fn test1() {
        let molecules = part1("test.txt");
        assert_eq!(4, molecules);
    }

    #[test]
    fn test2() {
        let steps = part2("test2.txt");
        assert_eq!(3, steps);
    }

    #[test]
    fn test3() {
        let steps = part2("test3.txt");
        assert_eq!(6, steps);
    }
}
