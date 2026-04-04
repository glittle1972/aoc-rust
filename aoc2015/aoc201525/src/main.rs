use std::time::Instant;

fn next(input: u64) -> u64 {
    return (input * 252533) % 33554393;
}

fn part1(row: u64, col: u64) -> u64 {
    let start = Instant::now();

    let mut value = 20151125;
    
    let mut r = 1u64;
    let mut c = 1u64;

    println!("{},{} = {}", r, c, value);

    loop {
        if r == row && c == col {
            break;
        }
        value = next(value);
        r -= 1;
        c += 1;
        if r == 0 {
            r = c;
            c = 1;
        }
        println!("{},{} = {}", r, c, value);
    }

    let dur = start.elapsed();
    println!("Duration = {:?}", dur);

    return value;
}

fn main() {
    let result1 = part1(2978, 3083);
    println!("result1 is {}", result1);
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test1() {
        let result1= part1(1, 6);
        assert_eq!(33511524, result1);
    }

    #[test]
    fn test1b() {
        let result1= part1(6, 2);
        assert_eq!(6796745, result1);
    }
}
