use std::fs;

fn main() {
    part1();
    part2();
}

fn part1(){

    let mut floor: i32 = 0;

    let contents: String = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    for c in contents.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
    }

    println!("Santa is on floor {floor}");
}

fn part2(){

    let mut floor: i32 = 0;
    let mut position: i32 = 0;

    let contents: String = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    for c in contents.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
        position += 1;
        if floor == -1 {
            break;
        }
    }

    println!("Santa reaches the basement after {position} steps.");
}
