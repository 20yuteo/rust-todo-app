use std::{collections::HashMap, io::{Read}, fs::File};

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo {map}),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e)
        }
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;

        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn get_file(self) -> File {
        return File::open("db.json").expect("file not found");
    }

    fn find_all(self) -> std::string::String {
        let mut contents = String::new();
        let mut file = self.get_file();
        let _ = file.read_to_string(&mut contents);
        return contents;
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let action: &String = &args[1];
    let mut item = String::new();
    if action != "all" {
        println!("input your items");
        std::io::stdin().read_line(&mut item).expect("failed to read line.");
    }

    let mut todo = Todo::new().expect("Initialisation of db failed");

    if action == "add" {
        todo.insert(item.trim().to_string());
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
    } else if action == "all" {
        let todo_list = todo.find_all();
        println!("todo_list: {}", todo_list);
    }
}
