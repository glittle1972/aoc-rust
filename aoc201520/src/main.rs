use std::time::Instant;

use factor::factor_include::factor_include;

fn main() {
    let house = part1(36000000);
    println!("result1 is {}", house);
}

fn part1(limit: u32) -> usize {
    let start = Instant::now();

    let mut house: usize = 800000;
    let mut presents: i64 = 0;
    while presents < (limit as i64) / 10 {
        let factors = factor_include(house as i64);
        presents = factors.iter().sum();
        println!("House {} gets {} presents", house, presents);
        house += 1;
    }
    
    let dur = start.elapsed();
    println!("Duration = {:?}", dur);

    return house;
}

