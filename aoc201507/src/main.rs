/**
 * For documentation, we have the following examples
 * 123 -> x
 * 456 -> y
 * x AND y -> d
 * x OR y -> e
 * x LSHIFT 2 -> f
 * y RSHIFT 2 -> g
 * NOT x -> h
 * NOT y -> i
 */
use std::collections::HashMap;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref re_line: Regex = Regex::new(r"(?<expr>.+) -> (?<symbol>[a-z]+)").unwrap();
    static ref re_assign: Regex = Regex::new(r"(?<left>^[0-9]+$|^[a-z]+$)").unwrap();
    static ref re_and: Regex = Regex::new(r"(?<left>[a-z]+|[0-9]+) AND (?<right>[a-z]+|[0-9]+)").unwrap();
    static ref re_or: Regex = Regex::new(r"(?<left>[a-z]+|[0-9]+) OR (?<right>[a-z]+|[0-9]+)").unwrap();
    static ref re_lshift: Regex = Regex::new(r"(?<left>[a-z]+|[0-9]+) LSHIFT (?<right>[a-z]+|[0-9]+)").unwrap();
    static ref re_rshift: Regex = Regex::new(r"(?<left>[a-z]+|[0-9]+) RSHIFT (?<right>[a-z]+|[0-9]+)").unwrap();
    static ref re_not: Regex = Regex::new(r"NOT (?<left>[a-z]+|[0-9]+)").unwrap();
}

fn main() {
    let res1 = part1("input.txt", "a");
    println!("Result 1 is {}", res1);
}

fn part1(filepath: &str, target: &str) -> u16 {
    // store a map of symbols against their respective operation - held as the raw string for now
    let mut dictionary = HashMap::new();
    let mut cache = HashMap::new();
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");
    
    for l in contents.lines() {
        let (symbol, expr) = parse_line(l);
        dictionary.insert(symbol, expr);
    }

    let result = get(&dictionary, &mut cache, target);
    return result;
}

fn parse_line(line: &str) -> (String, String) {
    let caps = re_line.captures(line).unwrap();

    return (String::from(&caps["symbol"]), String::from(&caps["expr"]));
}

fn get(dictionary: &HashMap<String, String>, cache: &mut HashMap<String, u16>, symbol: &str) -> u16 {
    let expr = dictionary.get(symbol).unwrap();
    
    let result = cache.get(symbol);
    if result.is_some() {
        return *result.unwrap();
    }

    let assign = re_assign.captures(&expr);
    if !assign.is_none() {
        let result = op_assign(dictionary, cache, &assign.unwrap()["left"]);
        println!("{} is {}", symbol, result);
        cache.insert(String::from(symbol), result);
        return result;
    }
    let not = re_not.captures(&expr);
    if !not.is_none() {
        let result = op_not(dictionary, cache, &not.unwrap()["left"]);
        println!("{} is {}", symbol, result);
        cache.insert(String::from(symbol), result);
        return result;
    }
    let and = re_and.captures(&expr);
    if !and.is_none() {
        let cap = and.unwrap();
        let result = op_and(dictionary, cache, &cap["left"], &cap["right"]);
        println!("{} is {}", symbol, result);
        cache.insert(String::from(symbol), result);
        return result;
    }
    let or = re_or.captures(&expr);
    if !or.is_none() {
        let cap = or.unwrap();
        let result = op_or(dictionary, cache, &cap["left"], &cap["right"]);
        println!("{} is {}", symbol, result);
        cache.insert(String::from(symbol), result);
        return result;
    }
    let lshift = re_lshift.captures(&expr);
    if !lshift.is_none() {
        let cap = lshift.unwrap();
        let result = op_lshift(dictionary, cache, &cap["left"], &cap["right"]);
        println!("{} is {}", symbol, result);
        cache.insert(String::from(symbol), result);
        return result;
    }
    let rshift = re_rshift.captures(&expr);
    if !rshift.is_none() {
        let cap = rshift.unwrap();
        let result = op_rshift(dictionary, cache, &cap["left"], &cap["right"]);
        println!("{} is {}", symbol, result);
        cache.insert(String::from(symbol), result);
        return result;
    }
    
    return 0;
}

fn op_assign(dictionary: &HashMap<String, String>, cache: &mut HashMap<String, u16>, left: &str) -> u16 {
    let l = left.parse::<u16>();
    // if we have an error, then assume this is a symbol and we
    // have to and get/calculate its value
    let lval = match l.is_ok() {
        true => l.unwrap(),
        _ => get(dictionary, cache, left),
    };

    let result = lval;
    return result;
}

fn op_not(dictionary: &HashMap<String, String>, cache: &mut HashMap<String, u16>, left: &str) -> u16 {
    let l = left.parse::<u16>();
    // if we have an error, then assume this is a symbol and we
    // have to and get/calculate its value
    let lval = match l.is_ok() {
        true => l.unwrap(),
        _ => get(dictionary, cache, left),
    };

    let result = !lval;
    return result;
}

fn op_and(dictionary: &HashMap<String, String>, cache: &mut HashMap<String, u16>, left: &str, right: &str) -> u16 {
    let l = left.parse::<u16>();
    // if we have an error, then assume this is a symbol and we
    // have to and get/calculate its value
    let lval = match l.is_ok() {
        true => l.unwrap(),
        _ => get(dictionary, cache, left),
    };
    let r = right.parse::<u16>();
    // if we have an error, then assume this is a symbol and we
    // have to and get/calculate its value
    let rval = match r.is_ok() {
        true => r.unwrap(),
        _ => get(dictionary, cache, right),
    };

    let result = lval & rval;
    return result;
}

fn op_or(dictionary: &HashMap<String, String>, cache: &mut HashMap<String, u16>, left: &str, right: &str) -> u16 {
    let l = left.parse::<u16>();
    // if we have an error, then assume this is a symbol and we
    // have to and get/calculate its value
    let lval = match l.is_ok() {
        true => l.unwrap(),
        _ => get(dictionary, cache, left),
    };
    let r = right.parse::<u16>();
    // if we have an error, then assume this is a symbol and we
    // have to and get/calculate its value
    let rval = match r.is_ok() {
        true => r.unwrap(),
        _ => get(dictionary, cache, right),
    };

    let result = lval | rval;
    return result;
}

fn op_lshift(dictionary: &HashMap<String, String>, cache: &mut HashMap<String, u16>, left: &str, right: &str) -> u16 {
    let l = left.parse::<u16>();
    // if we have an error, then assume this is a symbol and we
    // have to and get/calculate its value
    let lval = match l.is_ok() {
        true => l.unwrap(),
        _ => get(dictionary, cache, left),
    };
    let r = right.parse::<u16>();
    // if we have an error, then assume this is a symbol and we
    // have to and get/calculate its value
    let rval = match r.is_ok() {
        true => r.unwrap(),
        _ => get(dictionary, cache, right),
    };

    let result = lval << rval;
    return result;
}

fn op_rshift(dictionary: &HashMap<String, String>, cache: &mut HashMap<String, u16>, left: &str, right: &str) -> u16 {
    let l = left.parse::<u16>();
    // if we have an error, then assume this is a symbol and we
    // have to and get/calculate its value
    let lval = match l.is_ok() {
        true => l.unwrap(),
        _ => get(dictionary, cache, left),
    };
    let r = right.parse::<u16>();
    // if we have an error, then assume this is a symbol and we
    // have to and get/calculate its value
    let rval = match r.is_ok() {
        true => r.unwrap(),
        _ => get(dictionary, cache, right),
    };

    let result = lval >> rval;
    return result;
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test1x() {
        let res = part1("test.txt", "x");
        assert_eq!(123, res);
    }

    #[test]
    fn test1y() {
        let res = part1("test.txt", "y");
        assert_eq!(456, res);
    }

    #[test]
    fn test1h() {
        let res = part1("test.txt", "h");
        assert_eq!(65412, res);
    }

    #[test]
    fn test1i() {
        let res = part1("test.txt", "i");
        assert_eq!(65079, res);
    }

    #[test]
    fn test1d() {
        let res = part1("test.txt", "d");
        assert_eq!(72, res);
    }

    #[test]
    fn test1e() {
        let res = part1("test.txt", "e");
        assert_eq!(507, res);
    }

    #[test]
    fn test1f() {
        let res = part1("test.txt", "f");
        assert_eq!(492, res);
    }

    #[test]
    fn test1g() {
        let res = part1("test.txt", "g");
        assert_eq!(114, res);
    }

}