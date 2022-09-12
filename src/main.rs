#[macro_use]
extern crate rocket;
use serde::{Deserialize, Serialize};

mod database;
mod messages;
mod rooms;

use messages::*;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/message", routes![post_message, get_message])
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Message {
    user: usize,
    body: String,
}
