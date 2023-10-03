use std::{env, fs};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Entity {
    initiative: usize,
    name: String,
}

impl From<&str> for Entity {
    fn from(value: &str) -> Self {
        let fields: Vec<&str> = value.split(' ').collect();
        Self {
            initiative: fields[0].parse().unwrap(),
            name: fields[1].to_string(),
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

    let file_contents = fs::read_to_string(filepath).unwrap();
    let entity_strings = file_contents.lines();
    let mut entities: Vec<Entity> = entity_strings.map(|f| Entity::from(f)).collect();

    entities.sort_unstable();
    entities.reverse();

    for entity in entities {
        println!("{}\t({})", entity.name, entity.initiative);
    }
}
