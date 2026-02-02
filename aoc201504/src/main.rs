use regex::Regex;

fn main() {
    let res1 = part1(&"ckczppom".to_string());
    println!("Result1 is {}", res1);
}

fn part1(input: &String) -> i32 {
    let regex = Regex::new(r"^00000.+").unwrap();
    for i in 1..10000000 {
        let s = i.to_string();
        let str = input.to_owned() + &s;
        let digest = format!("{:x}", md5::compute(str));
        println!("i = {}, digest = {}", i, digest);
        if regex.is_match(&digest) {
            println!("Found a match at {}", i);
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