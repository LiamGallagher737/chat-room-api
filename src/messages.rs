use serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    user: usize,
    body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageData {
    user: usize,
    key: usize,
    body: String,
}

#[post("/<room>", format = "application/json", data = "<message>")]
pub fn post_message(room: usize, message: Json<MessageData>) -> String {
    format!("{:#?}", message)
}

#[get("/<room>/<count>")]
pub fn get_message(room: usize, count: usize) -> String {
    "cunt".into()
}

fn messages_database(room: usize) -> String {
    format!("/data/rooms/{}/messages.ron", room)
}
