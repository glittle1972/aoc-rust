use std::fs;

fn main() {
    part1();
}

fn part1(){

    let mut floor = 0;

    let contents = fs::read_to_string("input.txt")
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
