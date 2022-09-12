use ron::de::from_reader;
use serde::de::DeserializeOwned;
use std::fs::File;

static WORKING_PATH: &str =

pub fn create(path: String) {
    File::create(path);
}

pub fn save() {}

pub fn load<T: DeserializeOwned>(path: String) -> T {
    let file = File::open(&path).expect("Failed opening database file");
    let database: T = from_reader(file).expect("Failed reading database file");
    database
}
