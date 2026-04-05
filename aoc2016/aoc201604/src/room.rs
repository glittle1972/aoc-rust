use std::cmp::Ordering;
use std::collections::HashMap;
use regex::Regex;

const RE_ROOM: &str = r"^(?<name>[a-z-]+)-(?<sid>[0-9]+)\[(?<chk>[a-z]+)\]";

pub struct Room<'a> {
    pub name: &'a str,
    pub sid: usize,
    pub chk: &'a str
}

impl<'a> Room<'a> {
    pub fn new(string: &'a str) -> Room<'a> {
        let re_room = Regex::new(RE_ROOM).unwrap();

        let caps = re_room.captures(string).unwrap();
        let name = caps.name("name").unwrap().as_str();
        let sid = caps.name("sid").unwrap().as_str().parse::<usize>().unwrap();
        let chk = caps.name("chk").unwrap().as_str();

        Room { name, sid, chk }
    }

    fn calculate_checksum(&self) -> String {
        let mut hash: HashMap<char, usize> = HashMap::new();
        for c in self.name.chars() {
            if c != '-' {
                let count = hash.entry(c).or_insert(0);
                *count += 1;
            }
        }
        let mut vec: Vec<(&char, &usize)> = hash.iter().collect();
        vec.sort_by(|a, b| if a.1 == b.1 {
            a.0.cmp(b.0)
        } else {
            a.1.cmp(b.1).reverse()
        });
        let mut result = String::new();
        for c in vec.iter().take(5) {
            result.push(*c.0);
        }

        result
    }

    pub fn check(&self) -> bool {
        self.chk.cmp(&self.calculate_checksum()) == Ordering::Equal
    }

    fn decrypt_char(&self, b: &u8) -> u8 {
        if *b == 45 {
            return 32;
        }
        
        let mut a = *b - 97;
        let shift = (self.sid % 26) as u8;
        a += shift;
        a %= 26;
        a += 97;

        a
    }
    
    pub fn decrypt(&self) -> String {
        let mut result = String::new();
        for b in self.name.as_bytes() {
            result.push(self.decrypt_char(b) as char);
        }
        result
    }
}