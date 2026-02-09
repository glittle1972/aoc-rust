use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref re: Regex = Regex::new(r"(1+|2+|3+|4+|5+|6+|7+|8+|9+)").unwrap();
}

fn main() {
    let result = part1("1113222113", 40);
    println!("Result is {} with length {}", result, result.len());
    let result2 = part1("1113222113", 50);
    println!("Result 2 is {} with length {}", result2, result2.len());
}

fn part1(input: &str, iter: usize) -> String {
    let mut result = String::from(input);
    for i in 0..iter {
        if i < 10 {
            print!("{} becomes ", result);
        }
        result = parse(result);
        if i < 10 {
            println!("{}", result);
        }
    }
    return result;
}

fn parse(input: String) -> String {
    let mut result = String::new();
    
    let matches: Vec<_> = re.find_iter(&input).map(|m| m.as_str()).collect();
    for mtch in matches {
        let exp = expand(mtch);
        result = format!("{}{}", result, exp);
    }
    return result;
}

fn expand(expr: &str) -> String {
    let len = expr.len();
    let num = expr.chars().next().unwrap_or('0');
    return format!("{}{}", len, num);
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::expand;

    #[test]
    fn test1() {
        let result = part1("1", 5);
        assert_eq!("312211", result);
    }

    #[test]
    fn test111() {
        let result = expand("111");
        assert_eq!("31", result);
    }
}