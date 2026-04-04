use std::fs;
use std::time::Instant;

use regex::Regex;

#[derive(Debug)]
struct Sue {
    index: usize,
    children: usize,
    cats: usize,
    samoyeds: usize,
    pomeranians: usize,
    akitas: usize,
    vizslas: usize,
    goldfish: usize,
    trees: usize,
    cars: usize,
    perfumes: usize,
}

impl Default for Sue {
    fn default() -> Sue { 
        Sue {
            index: 0,
            children: std::usize::MAX,
            cats: std::usize::MAX,
            samoyeds: std::usize::MAX,
            pomeranians: std::usize::MAX,
            akitas: std::usize::MAX,
            vizslas: std::usize::MAX,
            goldfish: std::usize::MAX,
            trees: std::usize::MAX,
            cars: std::usize::MAX,
            perfumes: std::usize::MAX,
        }
    }
}

fn main() {
    let result1 = part1("input.txt", false);
    println!("result1 is {}", result1);
    let result2 = part1("input.txt", true);
    println!("result2 is {}", result2);
}

fn part1(filepath: &str, retroencabulator: bool) -> usize {
    const COMPOSITION: Sue = Sue { index: 0,
        children: 3, cats: 7, samoyeds: 2, pomeranians: 3, 
        akitas: 0 ,vizslas: 0, goldfish: 5, trees: 3,
        cars: 2, perfumes: 1 };
    
    let start = Instant::now();

    let mut sues: Vec<Sue> = vec![];
    
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");    
    parse_lines(contents, &mut sues);
    dbg!(&sues);

    for sue in sues {
        if compare_sues(&sue, &COMPOSITION, retroencabulator) {
            return sue.index;
        }
    }

    let dur = start.elapsed();
    println!("Duration = {:?}", dur);

    return 0;
}

fn compare_sues(source: &Sue, target: &Sue, retroencabulator: bool) -> bool {
    if !retroencabulator {
        return (source.children == std::usize::MAX || source.children == target.children) &&
            (source.cats == std::usize::MAX || source.cats == target.cats) && 
            (source.samoyeds == std::usize::MAX || source.samoyeds == target.samoyeds) && 
            (source.pomeranians == std::usize::MAX || source.pomeranians == target.pomeranians) && 
            (source.akitas == std::usize::MAX || source.akitas == target.akitas) && 
            (source.vizslas == std::usize::MAX || source.vizslas == target.vizslas) && 
            (source.goldfish == std::usize::MAX || source.goldfish == target.goldfish) && 
            (source.trees == std::usize::MAX || source.trees == target.trees) && 
            (source.cars == std::usize::MAX || source.cars == target.cars) && 
            (source.perfumes == std::usize::MAX || source.perfumes == target.perfumes);
    } else {
        return (source.children == std::usize::MAX || source.children == target.children) &&
            (source.cats == std::usize::MAX || source.cats > target.cats) && 
            (source.samoyeds == std::usize::MAX || source.samoyeds == target.samoyeds) && 
            (source.pomeranians == std::usize::MAX || source.pomeranians < target.pomeranians) && 
            (source.akitas == std::usize::MAX || source.akitas == target.akitas) && 
            (source.vizslas == std::usize::MAX || source.vizslas == target.vizslas) && 
            (source.goldfish == std::usize::MAX || source.goldfish < target.goldfish) && 
            (source.trees == std::usize::MAX || source.trees > target.trees) && 
            (source.cars == std::usize::MAX || source.cars == target.cars) && 
            (source.perfumes == std::usize::MAX || source.perfumes == target.perfumes);
    }
}

fn parse_lines(lines: String, sues: &mut Vec<Sue>) {
    let re_index = Regex::new(r"Sue ([0-9]+):").unwrap();
    let re_children = Regex::new(r"children: ([0-9]+)").unwrap();
    let re_cats = Regex::new(r"cats: ([0-9]+)").unwrap();
    let re_samoyeds = Regex::new(r"samoyeds: ([0-9]+)").unwrap();
    let re_pomeranians = Regex::new(r"pomeranians: ([0-9]+)").unwrap();
    let re_akitas = Regex::new(r"akitas: ([0-9]+)").unwrap();
    let re_vizslas = Regex::new(r"vizslas: ([0-9]+)").unwrap();
    let re_goldfish = Regex::new(r"goldfish: ([0-9]+)").unwrap();
    let re_trees = Regex::new(r"trees: ([0-9]+)").unwrap();
    let re_cars = Regex::new(r"cars: ([0-9]+)").unwrap();
    let re_perfumes = Regex::new(r"perfumes: ([0-9]+)").unwrap();
    for line in lines.lines() {
        let mut sue = Sue { index: 0, ..Default::default() };
        let index = re_index.captures(line).unwrap();
        let i = index[1].parse::<usize>().unwrap();
        sue.index = i;
        if re_children.is_match(line) {
            sue.children = re_children.captures(line).unwrap()[1].parse::<usize>().unwrap();
        }
        if re_cats.is_match(line) {
            sue.cats = re_cats.captures(line).unwrap()[1].parse::<usize>().unwrap();
        }
        if re_samoyeds.is_match(line) {
            sue.samoyeds = re_samoyeds.captures(line).unwrap()[1].parse::<usize>().unwrap();
        }
        if re_pomeranians.is_match(line) {
            sue.pomeranians = re_pomeranians.captures(line).unwrap()[1].parse::<usize>().unwrap();
        }
        if re_akitas.is_match(line) {
            sue.akitas = re_akitas.captures(line).unwrap()[1].parse::<usize>().unwrap();
        }
        if re_vizslas.is_match(line) {
            sue.vizslas = re_vizslas.captures(line).unwrap()[1].parse::<usize>().unwrap();
        }
        if re_goldfish.is_match(line) {
            sue.goldfish = re_goldfish.captures(line).unwrap()[1].parse::<usize>().unwrap();
        }
        if re_trees.is_match(line) {
            sue.trees = re_trees.captures(line).unwrap()[1].parse::<usize>().unwrap();
        }
        if re_cars.is_match(line) {
            sue.cars = re_cars.captures(line).unwrap()[1].parse::<usize>().unwrap();
        }
        if re_perfumes.is_match(line) {
            sue.perfumes = re_perfumes.captures(line).unwrap()[1].parse::<usize>().unwrap();
        }
        sues.push(sue);
    }
}

