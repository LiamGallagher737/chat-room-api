#[macro_use]
extern crate rocket;

mod database;
mod messages;
mod rooms;
mod users;

use messages::*;
use users::*;

#[launch]
fn rocket() -> _ {
    users::init();
    rocket::build()
        .mount("/message", routes![post_message, get_message])
        .mount("/user", routes![post_user])
}
