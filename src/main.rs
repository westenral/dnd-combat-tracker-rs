use std::{env, fs};

struct Entity {
    name: String,
    initiative: usize,
    health: isize,
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.initiative == other.initiative
    }

    fn ne(&self, other: &Self) -> bool {
        self.initiative != other.initiative
    }
}

impl PartialOrd for Entity {
    fn lt(&self, other: &Self) -> bool {
        self.initiative < other.initiative
    }

    fn le(&self, other: &Self) -> bool {
        self.initiative <= other.initiative
    }

    fn gt(&self, other: &Self) -> bool {
        self.initiative > other.initiative
    }

    fn ge(&self, other: &Self) -> bool {
        self.initiative >= other.initiative
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.initiative > other.initiative {
            return Some(std::cmp::Ordering::Greater);
        } else if self.initiative < other.initiative {
            return Some(std::cmp::Ordering::Less);
        } else {
            return Some(std::cmp::Ordering::Equal);
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let filepath;
    if let Some(f) = args.get(1) {
        filepath = f.clone();
    } else {
        filepath = String::from("inits.txt");
    }

    let entities;
    if let Ok(t) = fs::read_to_string(filepath) {
        entities = t.lines();
    } else {
        return;
    }
}
