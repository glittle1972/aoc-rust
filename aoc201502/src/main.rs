use std::{cmp::min, fs};

fn main() {
    part1();
}

fn part1(){

    let mut total = 0;

    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    for l in contents.lines() {
        let dims: Vec<i32> = l
            .split('x')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let l = dims[0];
        let w = dims[1];
        let h = dims[2];

        let top = top(l, w);
        let left = left(w, h);
        let front = front(l, h);
        let mut extra = min(top, left);
        extra = min(extra, front);

        let area = 2*top + 2*left + 2*front + top.min(left).min(front);

        println!("Area for present {}x{}x{} is {}", l, w, h, area);
        total += area;
    }
    println!("Total area is {}.", total);
}

fn top(l: i32, w: i32) -> i32 {
    return l * w;
}

fn left(w: i32, h: i32) -> i32 {
    return w * h;
}

fn front(l: i32, h: i32) -> i32 {
    return l * h;
}