use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::Error;

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().expect("key was not there");
    let value = args.next().unwrap();

    println!("The key is '{}' and the value is '{}'", key, value);

    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents).unwrap();
    
    let database = Database::new();
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, Error> {
        let contents = std::fs::read_to_string("kv.db")?;
        
        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => {
        //         return Result::Err(error)
        //     }
        // };

        let mut map = HashMap::new();

        for line in contents.lines() {
            let mut chunks = line.splitn(2, "\t");
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database {
            map: map,
        })
    }
}
