use regex::Regex;
use std::fs;

enum Op {
    On,
    Toggle,
    Off,
}

fn main() {
    let res1 = part1("input.txt");
    println!("Result 1 is {}", res1);
}

fn part1(filepath: &str) -> i32 {
    let mut lights = [false; 1000000];
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");
    for l in contents.lines() {
        let (op, x1, y1, x2, y2) = parse_line(l);
        println!("({},{}) -> ({},{})", x1, y1, x2, y2);
        switch(&mut lights, op, x1, y1, x2, y2);
    }
    return count(&lights);
}

fn count(lights: &[bool]) -> i32 {
    let mut result = 0;
    for b in lights.into_iter() {
        if *b {
            result += 1;
        }
    }
    return result;
}

fn switch(lights: &mut[bool], op: Op, x1: usize, y1: usize, x2: usize, y2: usize) {
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

    #[test]
    fn test1() {
        let res = part1("test.txt");
        assert_eq!(998996, res);
    }

}