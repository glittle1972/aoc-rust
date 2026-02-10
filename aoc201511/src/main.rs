use std::time::Instant;

// We don't need any sequences with i, o, or l
static THREE: [&str; 15] = [
    "abc", "bcd", "cde", "def", "efg",
    "fgh", "pqr", "qrs", "rst", "stu", 
    "tuv", "uvw", "vwx", "wxy", "xyz" ];
static PAIR: [&str; 23] = [
    "aa", "bb", "cc", "dd", "ee", "ff", "gg", "hh", "jj",
    "kk", "mm", "nn", "pp", "qq", "rr", "ss", "tt",
    "uu", "vv", "ww", "xx", "yy", "zz" ];
static IOL: [&str; 3] = [ "i", "o", "l" ];
fn main() {
    let mut password = String::from("hepxcrrq");
    part1(&mut password);
    println!("New password is {} ", password);
    part1(&mut password);
    println!("Next password is {} ", password);
}

fn part1(input: &mut String) {
    let start = Instant::now();

    loop {
        increment(input);
        if !test_iol(input) && test_three(input) && test_pair(input) {
            break;
        }
    }

    let dur = start.elapsed();
    println!("Duration = {:?}", dur);
}

fn increment(password: &mut String) {
    for pos in (0..password.len()).rev() {
        let c = password.chars().nth(pos).unwrap_or('0');
        if c < 'z' {
            let n = String::from(std::char::from_u32(c as u32 + 1).unwrap());
            password.replace_range(pos..pos+1, &n);
            break;
        } else {
            password.replace_range(pos..pos+1, "a");
        }
    }
}

fn test_three(password: &str) -> bool {
    for s in THREE {
        if password.contains(s) {
            return true;
        }
    }
    return false;
}

fn test_pair(password: &str) -> bool {
    let mut count = 0;
    for s in PAIR {
        if password.contains(s) {
            if count > 0 {
                return true;
            }
            count += 1;
        }
    }
    return false;
}

fn test_iol(password: &str) -> bool {
    for s in IOL {
        if password.contains(s) {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::increment;

    #[test]
    fn test1a() {
        let mut password = String::from("abcdefgh");
        part1(&mut password);
        assert_eq!("abcdffaa", password);
    }

    #[test]
    fn test1b() {
        let mut password = String::from("ghijklmn");
        part1(&mut password);
        assert_eq!("ghjaabcc", password);
    }

    #[test]
    fn inc_abc() {
        let mut password = String::from("abc");
        increment(&mut password);
        assert_eq!("abd", password);
    }

    #[test]
    fn inc_abz() {
        let mut password = String::from("abz");
        increment(&mut password);
        assert_eq!("aca", password);
    }

    #[test]
    fn inc_azf() {
        let mut password = String::from("azz");
        increment(&mut password);
        assert_eq!("baa", password);
    }
}