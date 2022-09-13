#[macro_use]
extern crate rocket;

mod database;
mod messages;
mod rooms;
mod users;

use messages::*;
use rooms::*;
use users::*;

#[launch]
fn rocket() -> _ {
    rooms::init();
    users::init();
    rocket::build()
        .mount("/message", routes![post_message, get_message])
        .mount("/room", routes![post_room])
        .mount("/user", routes![post_user])
}
