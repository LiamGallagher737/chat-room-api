use crate::database;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const USERS_DATABASE: &str = "/data/users/users.ron";
const ENCRPT_KEY: &str = "tete";

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    user: String,
    pass: String,
}

pub fn init() {
    database::create_checked::<HashMap<String, String>>(USERS_DATABASE.into());
}

#[post("/", format = "application/json", data = "<user>")]
pub fn post_user(user: Json<UserData>) {
    let mut users = database::read::<HashMap<String, String>>(USERS_DATABASE.into());
    if users.contains_key(&user.user) {
        return;
    }

    let mc = new_magic_crypt!(ENCRPT_KEY, 256);
    let pass_encrypted = mc.encrypt_str_to_base64(user.pass.clone());
    users.insert(user.user.clone(), pass_encrypted);
    database::save(USERS_DATABASE.into(), users);
}

pub fn verify(user: String, pass: String) -> bool {
    let users = database::read::<HashMap<String, String>>(USERS_DATABASE.into());
    if let Some(pass_encrypted) = users.get(&user) {
        let mc = new_magic_crypt!(ENCRPT_KEY, 256);
        if &mc.encrypt_str_to_base64(pass) == pass_encrypted {
            return true;
        }
    }
    false
}
