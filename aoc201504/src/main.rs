use regex::Regex;

fn main() {
    let res1 = part1(&"ckczppom".to_string());
    println!("Result1 is {}", res1);
    let res2 = part2(&"ckczppom".to_string());
    println!("Result2 is {}", res2);
}

fn part1(input: &String) -> i32 {
    return match_zeroes(&"5".to_string(), input);
}

fn part2(input: &String) -> i32 {
    return match_zeroes(&"6".to_string(), input);
}

fn match_zeroes(zeroes: &String, input: &String) -> i32 {
    let z: &str = zeroes;
    let re = "^0{".to_owned() + z + "}.*";
    let reref = &re;
    let regex = Regex::new(reref).unwrap();
    for i in 1..10000000 {
        let s = i.to_string();
        let str = input.to_owned() + &s;
        let digest = format!("{:x}", md5::compute(str));
        if regex.is_match(&digest) {
            println!("Found a match at {} ({})", i, digest);
            return i;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test1a() {
        let result1 = part1(&"abcdef".to_string());
        assert_eq!(result1, 609043);
    }

    #[test]
    fn test1b() {
        let result1 = part1(&"pqrstuv".to_string());
        assert_eq!(result1, 1048970);
    }

}