use std::{collections::HashMap, io::Read};
use std::str::FromStr;

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("out.txt")?;

        let mut content = String::new();
        f.read_to_string(&mut content)?;
        
        let mut map = HashMap::new();

        for entries in content.lines() {
            let mut values =  entries.split('\t');
            let key = values.next().expect("No Keys");
            let value = values.next().expect("No Values");

            map.insert(String::from(key), bool::from_str(value).unwrap());
        };
        Ok(Todo { map })
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record);
        }
        return std::fs::write("./out.txt", content);
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    println!("{:?}, {:?}", action, item);

let mut todo = Todo::new().expect("Initialisation of db failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved!"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("{} is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved!"),
                Err(why) => println!("An error occurred: {}",why),
            },
        }
    }
}
