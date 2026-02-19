use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use regex::Regex;

fn main() {
    let molecules = part1("input.txt");
    println!("result1 is {}", molecules);
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

    #[test]
    fn test1() {
        let molecules = part1("test.txt");
        assert_eq!(4, molecules);
    }

}
