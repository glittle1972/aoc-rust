use std::cmp::max;
use std::fs;
use std::time::Instant;

use regex::Regex;

#[derive(Debug)]
struct Ingredient {
    _name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    _calories: i32
}

fn main() {
    let result1 = part1("input.txt", 100);
    println!("result1 is {}", result1);
}

fn part1(filepath: &str, tsps: usize) -> i32 {
    let start = Instant::now();

    let mut ingredients: Vec<Ingredient> = vec![];
    
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");    
    parse_lines(contents, &mut ingredients);
    dbg!(&ingredients);

    let mut combination: Vec<usize> = vec![];
    let mut best = 0;
    evaluate_ingredients(&ingredients, tsps, &mut combination, &mut best);

    let dur = start.elapsed();
    println!("Duration = {:?}", dur);

    return best;
}

fn parse_lines(lines: String, ingredients: &mut Vec<Ingredient>) {
    // Candy: capacity 0, durability -1, flavor 0, texture 5, calories 8    
    let re = Regex::new(r"(?<name>[A-Z][a-z]+): capacity (?<cap>[-0-9]+), durability (?<dur>[-0-9]+), flavor (?<flav>[-0-9]+), texture (?<tex>[-0-9]+), calories (?<cal>[-0-9]+)").unwrap();
    for line in lines.lines() {
        let caps = re.captures(line).unwrap();
        let _name = String::from(&caps["name"]);
        let capacity = caps["cap"].parse::<i32>().unwrap();
        let durability = caps["dur"].parse::<i32>().unwrap();
        let flavor = caps["flav"].parse::<i32>().unwrap();
        let texture = caps["tex"].parse::<i32>().unwrap();
        let _calories = caps["cal"].parse::<i32>().unwrap();
        ingredients.push(Ingredient { _name, capacity, durability, flavor, texture, _calories });
    }
}

/**
 * I think we need something recursive. Increase the number of the first thing,
 * then call again with a slice having the first thing removed, and the total
 * reduced by number of first thing.
 * 
 * There might be some caching we can do later - there will be some repition of
 * quantities of pairs of ingredients.
 */
fn evaluate_ingredients(ingredients: &[Ingredient], tsps: usize, combination: &mut Vec<usize>, best: &mut i32) {

    // What's our stop condition? combination has one space left
    if ingredients.len() - combination.len() == 1 {
        combination.push(tsps);
        let score = calc_score(ingredients, combination);
        *best = max(*best, score);
        combination.pop();
    } else {
        // We need to loop up to the remaining number of teaspoons, MINUS
        // the number of remaining spaces after this one. As we need to leave
        // at least 1xN teaspoons for the following ingredients.
        let rem = ingredients.len() - combination.len();
        for i in 0..tsps + 1 - rem {
            combination.push(i + 1);
            let remaining_tsps = tsps - (i + 1);
            evaluate_ingredients(ingredients, remaining_tsps, combination, best);
            combination.pop();
        }
    }
}

fn calc_score(ingredients: &[Ingredient], combination: &[usize]) -> i32 {
    let mut capacity: i32 = 0;
    let mut durability: i32 = 0;
    let mut flavor: i32 = 0;
    let mut texture: i32 = 0;
    for index in 0..ingredients.len() {
        let i = &ingredients[index];
        let q = combination[index] as i32;
        capacity += i.capacity * q;
        durability += i.durability * q;
        flavor += i.flavor * q;
        texture += i.texture * q;
    }
    capacity = max(capacity, 0);
    durability = max(durability, 0);
    flavor = max(flavor, 0);
    texture = max(texture, 0);
    let result = capacity * durability * flavor * texture;
    println!("Score for {:?} is {}", combination, result);
    return result;
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test1() {
        let result1= part1("test.txt", 100);
        assert_eq!(62842880, result1);
    }

}
