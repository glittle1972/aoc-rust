use std::cmp::{min,max};
use std::time::Instant;

#[derive(Debug)]
struct Player {
    hp: i32,
    mana: i32,
    armor: i32,
}

#[derive(Debug)]
struct Boss {
    hp: i32,
    damage: i32,
}

#[derive(Clone,Copy,Debug)]
struct Spell {
    name: &'static str,
    cost: i32,
    damage: i32,
    health: i32,
    armor: i32,
    mana: i32,
    duration: i32,
}
/**
 * Attempt to cast a new spell. Returns true if the spell is cast (has enough mana) otherwise
 * returns false. If cast then the player's mana is updated and the active_spells list is updated.
 */
fn cast(level: usize, player: &mut Player, spell: &Spell, active_spells: &mut Vec<Spell>) -> bool {
    if player.mana < spell.cost {
        println!("{}. Cannot cast {} because mana {} is less than cost {}.", level, spell.name, player.mana, spell.cost);
        return false;
    }
    
    // We cannot cast duration spells if their effect is already active
    if spell.duration > 1 {
        if spell.armor > 0 && active_spells.iter().map(|s| s.armor).sum::<i32>() > 0 {
            println!("{}. Cannot cast {} because armor is already in effect.", level, spell.name);
            return false;
        }
        if spell.damage > 0 && active_spells.iter().map(|s| s.damage).sum::<i32>() > 0 {
            println!("{}. Cannot cast {} because damage is already in effect.", level, spell.name);
            return false;
        }
        if spell.health > 0 && active_spells.iter().map(|s| s.health).sum::<i32>() > 0 {
            println!("{}. Cannot cast {} because health is already in effect.", level, spell.name);
            return false;
        }
        if spell.mana > 0 && active_spells.iter().map(|s| s.mana).sum::<i32>() > 0 {
            println!("{}. Cannot cast {} because mana is already in effect.", level, spell.name);
            return false;
        }
    } 

    // OK to proceed
    player.mana -= spell.cost;
    // activate any armor bonus now
    player.armor += spell.armor;

    active_spells.push(spell.clone());
    println!("{}. Casting {:?}, player now {:?}.", level, spell, player);
    
    return true;
}

fn effect(level: usize, player: &mut Player, boss: &mut Boss, active_spells: &mut Vec<Spell>) {
    for active in &mut *active_spells {
        if active.duration > 0 {
            println!("{}. Applying {:?}", level, active);
            boss.hp -= active.damage;
//            player.armor += active.armor;
            player.hp += active.health;
            player.mana += active.mana;
            active.duration -= 1;
        }
        // remove armor if spell has expired
        if active.duration == 0 {
            player.armor -= active.armor;
        }
    }
    println!("{}. {:?}, {:?}", level, player, boss);

    // Get rid of any expired spells
    active_spells.retain(|s| s.duration > 0);
}

fn battle(level: usize, player: &mut Player, boss: &mut Boss, active_spells: &mut Vec<Spell>, spells: &Vec<Spell>, spent: &mut i32, minimum_spend: &mut i32) {
    println!("{}. New round: {:?} vs {:?}, ", level, player, boss);

    // First - apply any active spells
    effect(level, player, boss, active_spells);

    // Check if the boss is dead
    if boss.hp <= 0 { 
       *minimum_spend = min(*minimum_spend, *spent);
       println!("{}. Won with minimum spend {}.", level, minimum_spend);
       return;
    }

    // Now choose what to cast
    for spell in spells {
        // Create a copy of player and boss (and anything else that needs undone) for each loop
        let mut new_player = Player { hp: player.hp, mana: player.mana, armor: player.armor };
        let mut new_boss = Boss { hp: boss.hp, damage: boss.damage };
        let mut new_active = active_spells.to_owned();

        // Cast the next spell
        if cast(level, &mut new_player, spell, &mut new_active) {

            let mut new_spent = *spent + spell.cost;

            if new_spent >= *minimum_spend {
                println!("{}. Prune because spend of {} is already more than minimum {}", level, new_spent, minimum_spend);
                continue;
            }

            // Boss's turn - apply any active spells
            effect(level, &mut new_player, &mut new_boss, &mut new_active);

            if new_boss.hp <= 0 { 
                *minimum_spend = min(*minimum_spend, new_spent);
                println!("{}. Won with minimum spend {}.", level, minimum_spend);
                continue;
            }
            // Boss attacks
            new_player.hp -= max(1, new_boss.damage - new_player.armor);
            println!("{}. After boss attack {:?}.", level, new_player);
            if new_player.hp <= 0 { 
                println!("{}. Boss won!", level);
                continue;
            }

            // Both still alive, have another round
            battle(level + 1, &mut new_player, &mut new_boss, &mut new_active, spells, &mut new_spent, minimum_spend);
        }
    }
}

fn part1(init_player: &Player, init_boss: &Boss) -> i32 {
    let start = Instant::now();

    let spells: Vec<Spell> = vec![ 
        Spell { name: "Magic Missile", cost: 53, damage: 4, health: 0, armor: 0, mana: 0, duration: 1 }, 
        Spell { name: "Drain", cost: 73, damage: 2, health: 2, armor: 0, mana: 0, duration: 1 }, 
        Spell { name: "Shield", cost: 113, damage: 0, health: 0, armor: 7, mana: 0, duration: 6 }, 
        Spell { name: "Poison", cost: 173, damage: 3, health: 0, armor: 0, mana: 0, duration: 6 }, 
        Spell { name: "Recharge", cost: 229, damage: 0, health: 0, armor: 0, mana: 101, duration: 5 }, 
    ];

    let mut active_spells = vec![];
    
    let mut boss = Boss { hp: init_boss.hp, damage: init_boss.damage };
    let mut player = Player { hp: init_player.hp,
        mana: init_player.mana,
        armor: init_player.armor
    };
    let mut spend = 0;
    let mut minimum_spend = std::i32::MAX;

    battle(1, &mut player, &mut boss, &mut active_spells, &spells, &mut spend, &mut minimum_spend);
    
    let dur = start.elapsed();
    println!("Duration = {:?}", dur);

    return minimum_spend;
}

fn main() {
    const BOSS: Boss = Boss { hp: 51, damage: 9 };
    const PLAYER: Player = Player { hp: 50, mana: 500, armor: 0 };
    
    let min = part1(&PLAYER, &BOSS);
    println!("min to win is {}", min);
}

#[cfg(test)]
mod tests {
    use super::Player;
    use super::Boss;
    use super::part1;

    #[test]
    fn test1() {
        const PLAYER: Player = Player { hp: 10, mana: 250, armor: 0 };
        const BOSS: Boss = Boss { hp: 13, damage: 8 };
        let min = part1(&PLAYER, &BOSS);
        assert_eq!(226, min);
    }

    #[test]
    fn test1b() {
        const PLAYER: Player = Player { hp: 10, mana: 250, armor: 0 };
        const BOSS: Boss = Boss { hp: 14, damage: 8 };
        let min = part1(&PLAYER, &BOSS);
        assert_eq!(641, min);
    }
}