use std::{collections::HashMap, error};

fn main() {
    let mut arguments = std::env::args().skip(1);

    let key = arguments.next().expect("key was not there");
    let value = arguments.next().unwrap();

    println!("The key is '{}' and the value is '{}'", key, value);

    let contents = format!("{}\t{}", key, value);
    std::fs::write("kv.dv", contents).unwrap();

    // create our database
    let database = Database::new().expect("creating database failed");
}

// file will look like this.

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // read the kv.db file
        // let contents = match std::fs::read_to_string("kv.dv") {
        //     Ok(c) => c,
        //     Err(error) => {
        //         return Err(error);
        //     }
        // };
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
}
