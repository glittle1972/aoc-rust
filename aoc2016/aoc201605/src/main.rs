fn main() {
    let res1 = part1("reyedfim");
    println!("Result1 is {}", res1);
    let res2 = part2("reyedfim");
    println!("Result2 is {}", res2);
}

fn part1(input: &str) -> String {
    let mut result = String::new();
    let mut index = 0;

    while result.len() < 8 {
        match next_char(input, index) {
            Some(c) => result.push(c.0),
            None => ()
        }
        index += 1;
    }
    
    result
}

fn part2(input: &str) -> String {
    let mut result = String::from("        ");
    let mut index = 0;

    while result.contains(" ") {
        match next_char(input, index) {
            Some((pos, repl)) => {
                match pos.to_digit(10) {
                    Some(pos) => {
                        let pos = pos as usize;
                        match result.chars().nth(pos) {
                            Some(c) => {
                                if c == ' ' {
                                    result.replace_range(pos..=pos, &String::from(repl));
                                    println!("{}", result);
                                }
                            },
                            None => ()
                        }
                    },
                    None => ()
                }
            },
            None => ()
        }
        index += 1;
    }
    
    result
}

fn next_char(input: &str, index: i32) -> Option<(char, char)> {
    let str = format!("{}{}", input, index);
    let hash = format!("{:x}", md5::compute(str));
    if hash.starts_with("00000") {
        println!("Index {} results in hash {}", index, &hash);
        Some((hash.chars().nth(5).unwrap(), hash.chars().nth(6).unwrap()))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    #[test]
    fn test1() {
        let result1 = part1("abc");
        assert_eq!(result1, "18f47a30");
    }

    #[test]
    fn test2() {
        let result2 = part2("abc");
        assert_eq!(result2, "05ace8e3");
    }
}