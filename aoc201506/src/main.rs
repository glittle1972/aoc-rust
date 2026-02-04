use regex::Regex;
use std::{cmp,fs};

enum Op {
    On,
    Toggle,
    Off,
}

fn main() {
    let res1 = part1("input.txt");
    println!("Result 1 is {}", res1);
    let res2 = part2("input.txt");
    println!("Result 2 is {}", res2);
}

fn part1(filepath: &str) -> i32 {
    let mut lights = vec![false; 1000000];
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");
    for l in contents.lines() {
        let (op, x1, y1, x2, y2) = parse_line(l);
        switch1(&mut lights, op, x1, y1, x2, y2);
    }
    return count1(&lights);
}

fn count1(lights: &[bool]) -> i32 {
    let mut result = 0;
    for b in lights.into_iter() {
        if *b {
            result += 1;
        }
    }
    return result;
}

fn switch1(lights: &mut[bool], op: Op, x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..x2+1 {
        for y in y1..y2+1 {
            lights[y * 1000 + x] = match op {
                Op::On => true,
                Op::Off => false,
                _ => !lights[y * 1000 + x] 
            }
        }
    }
}

fn part2(filepath: &str) -> i32 {
    let mut lights = vec![0; 1000000];
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");
    for l in contents.lines() {
        let (op, x1, y1, x2, y2) = parse_line(l);
        switch2(&mut lights, op, x1, y1, x2, y2);
    }
    return count2(&lights);
}

fn count2(lights: &[i32]) -> i32 {
    let mut result = 0;
    for v in lights.into_iter() {
        result += v;
    }
    return result;
}

fn switch2(lights: &mut[i32], op: Op, x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..x2+1 {
        for y in y1..y2+1 {
            lights[y * 1000 + x] = match op {
                Op::On => lights[y * 1000 + x] + 1,
                Op::Off => cmp::max(lights[y * 1000 + x] - 1, 0),
                _ => lights[y * 1000 + x] + 2 
            }
        }
    }
}
fn parse_line(line: &str) -> (Op, usize, usize, usize, usize) {
    let re = Regex::new(r"(?<op>turn on|toggle|turn off) (?<x1>[0-9]{1,3}),(?<y1>[0-9]{1,3}) through (?<x2>[0-9]{1,3}),(?<y2>[0-9]{1,3})").unwrap();
    let caps = re.captures(line).unwrap();

    let op = match &caps["op"] {
        "turn on" => Op::On,
        "turn off" => Op::Off,
        _ => Op::Toggle
    };
    let x1 = &caps["x1"];
    let y1 = &caps["y1"];
    let x2 = &caps["x2"];
    let y2 = &caps["y2"];

    return (op, x1.parse::<usize>().unwrap(), y1.parse::<usize>().unwrap(),
        x2.parse::<usize>().unwrap(), y2.parse::<usize>().unwrap());
}


#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    #[test]
    fn test1() {
        let res = part1("test1.txt");
        assert_eq!(998996, res);
    }

    #[test]
    fn test2() {
        let res = part2("test2.txt");
        assert_eq!(1, res);
    }

    #[test]
    fn test3() {
        let res = part2("test3.txt");
        assert_eq!(2000000, res);
    }

}