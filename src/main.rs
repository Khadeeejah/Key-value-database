use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

fn main() {
    let mut arguments = std::env::args().skip(1);

    let key = arguments.next().expect("key was not there");
    let value = arguments.next().unwrap();

    println!("The key is '{}' and the value is '{}'", key, value);

    // let contents = format!("{}\t{}", key, value);
    // std::fs::write("kv.dv", contents).unwrap();

    // check if the file exist and if it doesnt create a new file and write to it
    if !Path::new("./kv.dv").is_file() {
        File::create("kv.dv").unwrap();
    }

    // create our database
    let mut database = Database::new().expect("creating database failed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    database.flush().unwrap();
}

// file will look like this.

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        // instead of writing the above you can rewrite it like this below
        let contents = std::fs::read_to_string("kv.dv")?;
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");
            map.insert(key.to_owned(), value.to_owned());
        }
        // parse the string
        // populate our map
        Ok(Database { map: map })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(self) -> std::io::Result<()> {
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }
        std::fs::write("kv.dv", contents)
    }
}
