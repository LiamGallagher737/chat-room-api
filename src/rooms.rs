use crate::{database, messages, users};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Variables {
    counter: usize,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Room {
    owner: String,
}

#[derive(Serialize, Deserialize)]
pub struct RoomData {
    user: String,
    pass: String,
}

const VARIABLES_DATABASE: &str = "/data/server/room_variables.ron";

pub fn init() {
    database::create_checked::<Variables>(VARIABLES_DATABASE.into());
}

#[post("/", format = "application/json", data = "<room>")]
pub fn post_room(room: Json<RoomData>) {
    if !users::verify(room.user.clone(), room.pass.clone()) {
        return;
    }

    let mut variables = database::read::<Variables>(VARIABLES_DATABASE.into());
    let id = variables.counter + 1;
    variables.counter += 1;
    database::save(VARIABLES_DATABASE.into(), variables);

    database::create::<Room>(format!("/data/rooms/{}/data.ron", id));
    database::create::<Vec<String>>(format!("/data/rooms/{}/users.ron", id));
    database::create::<Vec<messages::Message>>(format!("/data/rooms/{}/messages.ron", id));

    database::save(
        format!("/data/rooms/{}/data.ron", id),
        Room {
            owner: room.user.clone(),
        },
    );
}
