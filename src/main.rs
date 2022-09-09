#[macro_use]
extern crate rocket;
use jammdb::DB;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

const DATABASE: &str = "/data.db";

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/message", routes![send, get])
}

#[post("/<room>/<user>", format = "json", data = "<message>")]
fn send(room: u128, user: u128, message: Json<String>) {
    let db = DB::open(MESSAGE_DATABASE).expect("Failed to open message database");
    let mut tx = db.tx(true);

    let 
}

#[get("/<room>/<count>")]
fn get(room: u128, count: u8) {
    
}

#[derive(Serialize, Deserialize)]
struct Message {
    user: u128,
    body: String,
}
