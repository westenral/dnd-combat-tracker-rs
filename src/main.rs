use std::{env, fs, io};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Entity {
    initiative: usize,
    name: String,
    health: Option<isize>,
    damage: usize,
}

impl From<&str> for Entity {
    fn from(value: &str) -> Self {
        let fields: Vec<&str> = value.split(' ').collect();
        let health = match fields.get(2) {
            Some(h) => Some(h.parse().unwrap()),
            None => None,
        };
        let damage = fields.get(3).unwrap_or(&"0").parse().unwrap();
        Self {
            initiative: fields[0].parse().unwrap(),
            name: fields[1].to_string(),
            health,
            damage,
        }
    }
}

fn search_for_entity_name(entities: &mut Vec<Entity>, target: String) -> Option<&mut Entity> {
    for entity in entities {
        if entity.name == target {
            return Some(entity);
        }
    }

    None
}

fn create_save_string(entities: &Vec<Entity>) -> String {
    let mut save_string = String::new();

    for entity in entities {
        let mut line = format!("{} {}", entity.initiative, entity.name);
        if let Some(h) = entity.health {
            line.push_str(format!(" {} {}", h, entity.damage).as_str());
        }
        save_string.push_str(&(line + "\n"));
    }

    save_string
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let filepath;
    if let Some(f) = args.get(1) {
        filepath = f.clone();
    } else {
        filepath = String::from("inits.txt");
    }

    println!("Loading {}", filepath);

    let file_contents = fs::read_to_string(filepath).unwrap();
    let entity_strings = file_contents.lines();
    let mut entities: Vec<Entity> = entity_strings.map(|f| Entity::from(f)).collect();

    entities.sort_unstable();
    entities.reverse();

    println!();
    for entity in &entities {
        println!(
            "{}\t{} init\t\t{} hp",
            entity.name,
            entity.initiative,
            entity.health.unwrap_or(0) - entity.damage as isize,
        );
    }
    println!();
    println!("\"dmg <name> <damage>\"\tdeal damage\n\"save <filepath>\"\toutput to file\n\"print\"\t\t\tprint current state\n\"quit\"\t\t\texit program");
    println!();

    'input_loop: loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut input = input.trim().split_whitespace();
        match input.next() {
            Some("dmg") => {
                let target_name = match input.next() {
                    Some(n) => n,
                    None => {
                        println!("Invalid target name");
                        continue 'input_loop;
                    }
                };

                // horrible horrible code, thx rust
                let damage = match input.next() {
                    Some(dstr) => match dstr.parse::<usize>() {
                        Ok(d) => d,
                        Err(_) => {
                            println!("Damage should be numerical");
                            continue 'input_loop;
                        }
                    },
                    None => {
                        println!("Please specify damage");
                        continue 'input_loop;
                    }
                };

                let target = search_for_entity_name(&mut entities, target_name.to_string());
                match target {
                    Some(t) => t.damage += damage,
                    None => {
                        println!("Target not found");
                        continue 'input_loop;
                    }
                }

                println!("{} took {} damage", target_name, damage);
            }
            Some("save") => {
                let save_filepath = match input.next() {
                    Some(f) => f,
                    None => {
                        println!("No filepath specified");
                        continue 'input_loop;
                    }
                };
                if let Ok(_) = fs::read(save_filepath) {
                    println!("File already exists; aborting");
                    continue 'input_loop;
                }

                fs::write(save_filepath, create_save_string(&entities)).unwrap();
            }
            Some("print") => {
                println!();
                for entity in &entities {
                    println!(
                        "{}\t{} init\t\t{} hp",
                        entity.name,
                        entity.initiative,
                        entity.health.unwrap_or(0) - entity.damage as isize,
                    );
                }
            }
            Some("quit") => break 'input_loop,
            Some(_) => println!("Invalid input"),
            None => {}
        }

        for (i, entity) in entities.clone().into_iter().enumerate() {
            if entity.health.is_some_and(|e| entity.damage as isize >= e) {
                println!("{} has died; removing from turn order", entity.name);
                entities.remove(i);
            }
        }

        println!();
    }
}
