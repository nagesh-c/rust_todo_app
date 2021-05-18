use std::env;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::io::Read;

// todo structure
struct Todo {
    // using builtin hashmap
    map: HashMap<String, bool>,
}

// implements the methods for the structure
// impl is specfic to the data type
impl Todo {

    fn new() -> Result<Todo, std::io::Error> {
        // ?; is a shorthand for error handling
        // if there is any error will return immediately
        // with an error. if ? is not used below the result will
        // be stored in f rather than exact file object
        // further reference to the f will be ResultObj.(...).
        let mut f = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open("db.txt")?;

        // Create a string buffer
        let mut content = String::new();

        // read the contents of file from the string buffer
        f.read_to_string(&mut content)?;

        // create an HashMap to store the contents in the file
        let mut map = HashMap::new();

        for entries in content.lines() {
            let mut values = entries.split('\t');
            let key = values.next().expect("No key");
            let val = values.next().expect("No value");
            // insert into Hashmap
            map.insert(String::from(key), bool::from_str(val).unwrap());
        }

        Ok(Todo{map})
    }

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

    // snippet to mark the todo as completed.
    fn complete(&mut self, key: &String) -> Option<()> {
        // this will get the mutable reference to the value of the key
        match self.map.get_mut(key) {
            Some(value) => Some(*value = false),
            None => None,
        }
    }
}

fn todo() {
    let action = env::args().nth(1).expect("Please specify an action");
    let item = env::args().nth(2).expect("Please specify an item");
    //println!("{:?} {:?} {:?}", action, item, env::args().len());
    
    let mut todo = Todo::new().expect("Initialization of db failed.");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved."),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' item not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo updated for item {}", item),
                Err(why) => println!("An error occurred: {}", why),
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    todo();
}
