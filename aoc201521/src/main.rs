use std::cmp::{min,max};
use std::time::Instant;

#[derive(Debug)]
struct Player {
    cost: i32,
    hp: i32,
    damage: i32,
    armor: i32,
}

struct Artefact {
    _name: &'static str,
    cost: i32,
    damage: i32,
    armor: i32,
}

/**
 * Return true of player1 wins
 */
fn battle(player1: &mut Player, player2: &mut Player) -> bool {
    print!("{:?} vs {:?}, ", player1, player2);
    loop {
        // player1 goes first
        let d1 = max(1, player1.damage - player2.armor);
        player2.hp -= d1;
        if player2.hp <= 0 {
            println!("player1 wins");
            return true;
        }
        // player2 responds
        let d2 = max(1, player2.damage - player1.armor);
        player1.hp -= d2;
        if player1.hp <= 0 {
            println!("player1 loses");
            return false;
        }
    }
}

fn part1() -> (i32, i32) {
    let start = Instant::now();

    const BOSS: Player = Player { cost: 0, hp: 100, damage: 8, armor: 2 };
    const INITIAL_HP: i32 = 100;

    let weapons: Vec<Artefact> = vec![ 
        Artefact { _name: "Dagger", cost: 8, damage: 4, armor: 0}, 
        Artefact { _name: "Shortsword", cost: 10, damage: 5, armor: 0}, 
        Artefact { _name: "Warhammer", cost: 25, damage: 6, armor: 0}, 
        Artefact { _name: "Longsword", cost: 40, damage: 7, armor: 0}, 
        Artefact { _name: "Greataxe", cost: 74, damage: 8, armor: 0}, 
    ];
    let armors: Vec<Artefact> = vec![ 
        Artefact { _name: "Optional", cost: 0, damage: 0, armor: 0},
        Artefact { _name: "Leather", cost: 13, damage: 0, armor: 1}, 
        Artefact { _name: "Chainmail", cost: 31, damage: 0, armor: 2}, 
        Artefact { _name: "Splintmail", cost: 53, damage: 0, armor: 3}, 
        Artefact { _name: "Bandedmail", cost: 75, damage: 0, armor: 4}, 
        Artefact { _name: "Platemail", cost: 102, damage: 0, armor: 5}, 
    ];
    let rings: Vec<Artefact> = vec![ 
        Artefact { _name: "Optional1", cost: 0, damage: 0, armor: 0}, 
        Artefact { _name: "Damage+1", cost: 25, damage: 1, armor: 0}, 
        Artefact { _name: "Damage+2", cost: 50, damage: 2, armor: 0}, 
        Artefact { _name: "Damage+3", cost: 100, damage: 3, armor: 0}, 
        Artefact { _name: "optional2", cost: 0, damage: 0, armor: 0}, 
        Artefact { _name: "Defense+1", cost: 20, damage: 0, armor: 1}, 
        Artefact { _name: "Defense+2", cost: 40, damage: 0, armor: 2}, 
        Artefact { _name: "Defense+3", cost: 80, damage: 0, armor: 3}, 
    ];

    // Four loops to determien each combination
    let (mut minimum_cost, mut maximum_cost) = (i32::MAX, i32::MIN);
    for weapon in weapons {
        for armor in &armors {
            for r1 in 0..rings.len() - 1 {
                for r2 in (r1 + 1)..rings.len() {
                    let ring1 = &rings[r1];
                    let ring2 = &rings[r2];
                    let mut player1 = Player { 
                        cost: weapon.cost + armor.cost + ring1.cost + ring2.cost,
                        hp: INITIAL_HP, 
                        damage: weapon.damage + armor.damage + ring1.damage + ring2.damage,
                        armor: weapon.armor + armor.armor + ring1.armor + ring2.armor
                    };
                    let mut player2 = Player { cost: BOSS.cost, hp: BOSS.hp, damage: BOSS.damage, armor: BOSS.armor };
                    if battle(&mut player1, &mut player2) {
                        minimum_cost = min(minimum_cost, player1.cost);
                    } else {
                        maximum_cost = max(maximum_cost, player1.cost);
                    }
                }
            }
        }
    }
    
    let dur = start.elapsed();
    println!("Duration = {:?}", dur);

    return (minimum_cost, maximum_cost);
}

fn main() {
    let (min, max) = part1();
    println!("min to win is {}", min);
    println!("max to lose is {}", max);
}

