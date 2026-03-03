use std::time::Instant;

use factor::factor_include::factor_include;

fn main() {
//    let house = part1(36000000);
//    println!("result1 is {}", house);
    let house2 = part2(36000000);
    println!("result2 is {}", house2);
}

fn part1(limit: u32) -> usize {
    let start = Instant::now();

    let mut house: usize = 1;
    let mut presents: i64 = 0;
    while presents < (limit as i64) / 10 {
        let factors = factor_include(house as i64);
        presents = factors.iter().sum();
        println!("House {} gets {} presents", house, presents);
        house += 1;
    }
    
    let dur = start.elapsed();
    println!("Duration = {:?}", dur);

    return house - 1;
}

fn part2(limit: u32) -> u32 {
    let start = Instant::now();

    let mut house: u32 = 800011;
    let mut presents: u32 = 0;
    while presents < limit {
        presents = 0;
        let factors: Vec<u32> = factor_include(house as i64).iter().map(|i| *i as u32).collect();
        for elf in factors {
            if house <= elf * 50 {
                presents += elf * 11;
            }
        }
        println!("House {} gets {} presents", house, presents);
        house += 1;
    }
    
    let dur = start.elapsed();
    println!("Duration = {:?}", dur);

    return house - 1;
}
