use std::fs;

mod room;

use room::Room;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Could not read file");
    
    let rooms: Vec<Room> = contents.lines().map(|s| Room::new(s)).collect();
    
    let result = part1(&rooms);
    println!("Result is {result}");
}

fn part1(rooms: &Vec<Room>) -> usize {

    let result = rooms.iter().filter(|r| r.check()).map(|r| r.sid).sum();
    
    result
}
