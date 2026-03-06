use std::fs;
use std::collections::HashMap;
use std::time::Instant;

use regex::Regex;

trait Op {
    /**
     * Update the registers' values as defined by the Op,
     * and return any offset (may be negative)
     */
    fn exec(&self, registers: &mut HashMap<char, i32>) -> i32;
}

struct Hlf {
    register: char,
}

impl Hlf {
    fn new(register: char) -> Box<dyn Op> {
        return Box::new(Hlf { register });
    }
}

impl Op for Hlf {
    fn exec(&self, registers: &mut HashMap<char, i32>) -> i32 {
        println!("Hlf.exec({})", self.register);
        registers.insert(self.register, registers.get(&self.register).unwrap() / 2);
        return 1;
    }
}

struct Tpl {
    register: char,
}

impl Tpl {
    fn new(register: char) -> Box<dyn Op> {
        return Box::new(Tpl { register });
    }
}

impl Op for Tpl {
    fn exec(&self, registers: &mut HashMap<char, i32>) -> i32 {
        println!("Tpl.exec({})", self.register);
        registers.insert(self.register, registers.get(&self.register).unwrap() * 3);
        return 1;
    }
}

struct Inc {
    register: char,
}

impl Inc {
    fn new(register: char) -> Box<dyn Op> {
        return Box::new(Inc { register });
    }
}

impl Op for Inc {
    fn exec(&self, registers: &mut HashMap<char, i32>) -> i32 {
        println!("Inc.exec({})", self.register);
        registers.insert(self.register, registers.get(&self.register).unwrap() + 1);
        return 1;
    }
}

struct Jmp {
    offset: i32,
}

impl Jmp {
    fn new(offset: i32) -> Box<dyn Op> {
        return Box::new(Jmp { offset });
    }
}

impl Op for Jmp {
    fn exec(&self, _registers: &mut HashMap<char, i32>) -> i32 {
        println!("Jmp.exec({})", self.offset);
        return self.offset;
    }
}

struct Jie {
    register: char,
    offset: i32,
}

impl Jie {
    fn new(register: char, offset: i32) -> Box<dyn Op> {
        return Box::new(Jie { register, offset });
    }
}

impl Op for Jie {
    fn exec(&self, registers: &mut HashMap<char, i32>) -> i32 {
        println!("Jie.exec({}, {})", self.register, self.offset);
        if registers.get(&self.register).unwrap() % 2 == 0 {
            return self.offset;
        } else {
            return 1;
        }
    }
}

struct Jio {
    register: char,
    offset: i32,
}

impl Jio {
    fn new(register: char, offset: i32) -> Box<dyn Op> {
        return Box::new(Jio { register, offset });
    }
}

impl Op for Jio {
    fn exec(&self, registers: &mut HashMap<char, i32>) -> i32 {
        println!("Jio.exec({}, {})", self.register, self.offset);
        if *registers.get(&self.register).unwrap() == 1 {
            return self.offset;
        } else {
            return 1;
        }
    }
}

struct Noop { }

impl Noop {
    fn new() -> Box<dyn Op> {
        return Box::new(Noop {});
    }
}

impl Op for Noop {
    fn exec(&self, _registers: &mut HashMap<char, i32>) -> i32 {
        println!("Noop.exec()");
        return 0;
    }
}

fn parse_lines(lines: String, ops: &mut Vec<Box<dyn Op>>) {
    let re = Regex::new(r"^(?<op>[a-z]{3}) (?<reg>[a-z]{0,1})(?:, ){0,1}(?<off>[+-][0-9]+){0,1}$").unwrap();
    for line in lines.lines() {
        let caps = re.captures(line).unwrap();
        let name = caps.name("op").map_or("noop", |m| m.as_str());
        let register = caps.name("reg").map_or('z', |m| m.as_str().chars().next().unwrap_or('z'));
        let offset = caps.name("off").map_or(0, |m| m.as_str().parse::<i32>().unwrap());
        let op = match name {
            "hlf" => Hlf::new(register),
            "tpl" => Tpl::new(register),
            "inc" => Inc::new(register),
            "jmp" => Jmp::new(offset),
            "jie" => Jie::new(register, offset),
            "jio" => Jio::new(register, offset),
            _ => Noop::new(),
        };
        ops.push(op);
    }
}

fn part1(filepath: &str) -> i32 {
    let start = Instant::now();

    let mut registers: HashMap<char, i32> = HashMap::new();
    registers.insert('a', 0);
    registers.insert('b', 0);
    let mut ops = vec![];

    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");    
    parse_lines(contents, &mut ops);

    let mut pos = 0;
    while pos < ops.len() {
        let offset = ops[pos].exec(&mut registers);
        let ipos = (pos as i32) + offset;
        pos = ipos.try_into().unwrap();
    }

    let dur = start.elapsed();
    println!("Duration = {:?}", dur);

    return *registers.get(&'b').unwrap();
}

fn main() {
    let result1 = part1("input.txt");
    println!("result1 is {}", result1);
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test1() {
        let result1= part1("test.txt");
        assert_eq!(0, result1);
    }

}
