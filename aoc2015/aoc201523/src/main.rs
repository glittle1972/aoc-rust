use std::{collections::HashMap, fs, time::Instant};

use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Op {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(i32),
    Jie(char, i32),
    Jio(char, i32),
}

impl Op {
    /// Execute the operation, updating `registers` and returning the instruction pointer offset.
    fn exec(&self, registers: &mut HashMap<char, i32>) -> i32 {
        match *self {
            Op::Hlf(r) => {
                let v = *registers.get(&r).unwrap_or(&0) / 2;
                registers.insert(r, v);
                1
            }
            Op::Tpl(r) => {
                let v = *registers.get(&r).unwrap_or(&0) * 3;
                registers.insert(r, v);
                1
            }
            Op::Inc(r) => {
                let v = *registers.get(&r).unwrap_or(&0) + 1;
                registers.insert(r, v);
                1
            }
            Op::Jmp(off) => off,
            Op::Jie(r, off) => {
                if registers.get(&r).unwrap_or(&0) % 2 == 0 {
                    off
                } else {
                    1
                }
            }
            Op::Jio(r, off) => {
                if *registers.get(&r).unwrap_or(&0) == 1 {
                    off
                } else {
                    1
                }
            }
        }
    }
}

fn parse_lines(lines: String, ops: &mut Vec<Op>) {
    let re = Regex::new(r"^(?P<op>[a-z]{3}) (?P<reg>[a-z]?)(?:, )?(?P<off>[+-][0-9]+)?$").unwrap();
    for line in lines.lines() {
        let caps = re.captures(line).unwrap();
        let name = caps.name("op").unwrap().as_str();
        let reg_opt = caps.name("reg").and_then(|m| m.as_str().chars().next());
        let off_opt = caps.name("off").map(|m| m.as_str().parse::<i32>().unwrap());

        let op = match name {
            "hlf" => match reg_opt { Some(r) => Op::Hlf(r), None => continue },
            "tpl" => match reg_opt { Some(r) => Op::Tpl(r), None => continue },
            "inc" => match reg_opt { Some(r) => Op::Inc(r), None => continue },
            "jmp" => Op::Jmp(off_opt.unwrap_or(0)),
            "jie" => match (reg_opt, off_opt) { (Some(r), Some(o)) => Op::Jie(r, o), _ => continue },
            "jio" => match (reg_opt, off_opt) { (Some(r), Some(o)) => Op::Jio(r, o), _ => continue },
            _ => continue,
        };

        ops.push(op);
    }
}

fn part1(filepath: &str, initial: (i32, i32)) -> i32 {
    let start = Instant::now();

    let mut registers: HashMap<char, i32> = HashMap::new();
    registers.insert('a', initial.0);
    registers.insert('b', initial.1);
    let mut ops: Vec<Op> = Vec::new();

    let contents = fs::read_to_string(filepath).expect("Could not read file");
    parse_lines(contents, &mut ops);

    let mut pos: i32 = 0;
    while (pos as usize) < ops.len() {
        let offset = ops[pos as usize].exec(&mut registers);
        pos += offset;
    }

    let dur = start.elapsed();
    println!("Duration = {:?}", dur);

    *registers.get(&'b').unwrap_or(&0)
}

fn main() {
    let result1 = part1("input.txt", (0, 0));
    println!("result1 is {}", result1);
    let result2 = part1("input.txt", (1, 0));
    println!("result2 is {}", result2);
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test1() {
        let result1 = part1("test.txt", (0, 0));
        assert_eq!(0, result1);
    }
}
