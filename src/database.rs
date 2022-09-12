use rocket::serde::json::to_string;
use ron::{
    de::from_reader,
    ser::{to_string_pretty, PrettyConfig},
};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    env,
    fs::{self, File},
    path::Path,
};

pub fn create_checked<T: Serialize + Default>(dir: String) {
    if !Path::new(&dir).exists() {
        create::<T>(dir);
    }
}

pub fn create<T: Serialize + Default>(dir: String) {
    let dir = &database_directory(dir);
    let path = Path::new(dir);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    File::create(path).unwrap();
    fs::write(path, to_string(&T::default()).unwrap()).unwrap();
}

pub fn read<T: DeserializeOwned>(path: String) -> T {
    let file = File::open(database_directory(path)).expect("Failed opening database file");
    let database: T = from_reader(file).expect("Failed reading database file");
    database
}

pub fn save<T: Serialize>(path: String, data: T) {
    fs::write(
        database_directory(path),
        to_string_pretty(&data, PrettyConfig::default()).unwrap(),
    )
    .unwrap();
}

fn database_directory(path: String) -> String {
    env::current_dir().unwrap().to_str().unwrap().to_owned() + &path
}
