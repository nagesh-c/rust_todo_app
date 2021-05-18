use std::env;
use std::collections::HashMap;

// todo structure
struct Todo {
    // using builtin hashmap
    map: HashMap<String, bool>,
}

// implements the methods for the structure
// impl is specfic to the data type
impl Todo {
    // self is borrowed here, because the string is
    // added the the main object, since its getting
    // updated hence the mut
    fn insert(&mut self, key: String) {
        // insert a new todo in the list
        self.map.insert(key, true);
    }

    // snippet to store the todo in the file
    fn save(self) -> Result<(), std::io::Error> {
        // this has to be mut because we will be extracting
        // the contents from the HashMap and store in it
        let mut content = String::new();
        // extract from the map and make it collection
        // beautify it and store it in the file
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record);
        }
        std::fs::write("db.txt", content)
    }
}

fn todo() {
    let action = env::args().nth(1).expect("Please specify an action");
    let item = env::args().nth(2).expect("Please specify an item");
    //println!("{:?} {:?} {:?}", action, item, env::args().len());
    
    let mut todo = Todo {
        map: HashMap::new(),
    };

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved."),
            Err(why) => println!("An error occurred: {}", why),
        }
    }
}

fn main() {
    println!("Hello, world!");
    todo();
}
