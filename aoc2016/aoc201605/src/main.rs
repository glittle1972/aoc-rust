fn main() {
    let res1 = part1("reyedfim");
    println!("Result1 is {}", res1);
}

fn part1(input: &str) -> String {
    let mut result = String::new();
    let mut index = 0;

    while result.len() < 8 {
        match next_char(input, index) {
            Some(c) => result.push(c),
            None => ()
        }
        index += 1;
    }
    
    result
}

fn next_char(input: &str, index: i32) -> Option<char> {
    let str = format!("{}{}", input, index);
    let hash = format!("{:x}", md5::compute(str));
    if hash.starts_with("00000") {
        println!("Index {} results in hash {}", index, &hash);
        hash.chars().nth(5)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test1a() {
        let result1 = part1("abc");
        assert_eq!(result1, "18f47a30");
    }
}