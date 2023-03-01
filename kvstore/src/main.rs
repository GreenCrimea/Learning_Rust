use std::{fs::{self, File}, collections::HashMap, path::Path};

fn main() {
    let file_path = Path::new("kv.db");

    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();

    println!("\n\nthe DB contains:\n");
    if file_path.exists() {

        let mut database = Database::new(file_path).expect("creating db failed");
        database.map.insert(key, value);
        let mut contents = String::new();

        for (key, value) in database.map {

            println!("{} : {}", key, value);
            contents.push_str(&format!("{}\t{}\n", key, value));
        }
        fs::write(file_path, contents).unwrap();

    } else {

        println!("{} : {}", key, value);
        File::create(file_path).expect("error creating file");
        fs::write(file_path, format!("{}\t{}\n", key, value)).unwrap();
    }
}


  
struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new(file_path: &Path) -> Result<Database, std::io::Error> {

        let mut map = HashMap::new();

        let contents = fs::read_to_string(file_path)?;

        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("corrupt database");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database { map })
    }
}