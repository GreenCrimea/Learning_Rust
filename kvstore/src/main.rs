use std::{fs::{self, OpenOptions, File}, collections::HashMap, path::Path, io::Write};

fn main() {
    let file_path = Path::new("kv.db");

    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();

    println!("\n\nthe DB contains:\n");
    if file_path.exists() {

        let mut database = Database::new(file_path).expect("creating db failed");
        database.map.insert(key, value);
        fs::remove_file(file_path).expect("failed to remove old file");
        create_file(file_path);

        for (key, value) in database.map {
            println!("{} : {}", key, value);
            write_keys_values(file_path, key, value).expect("failed to write to file");
        }

    } else {

        println!("{} : {}", key, value);
        create_file(file_path);
        write_keys_values(file_path, key, value).expect("failed to write to file");
    }
}



fn create_file(file_path: &Path) -> () {
    File::create(file_path).expect("error creating file");
}

fn write_keys_values(file_path: &Path, key: String, value: String) -> Result<(), std::io::Error> {
    let mut fileref = OpenOptions::new().append(true).open(file_path).expect("unable to open file");
    fileref.write_all(combine_keys_values(key, value).as_bytes())   
}

fn combine_keys_values(key: String, value: String) -> String {
    format!("{}\t{}\n", key, value)
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
        Ok(Database { map: map })
    }
}