use crate::{database, users};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    user: String,
    body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageData {
    user: String,
    pass: String,
    body: String,
}

#[post("/<room>", format = "application/json", data = "<message>")]
pub fn post_message(room: usize, message: Json<MessageData>) {
    database::create_checked::<Vec<Message>>(messages_database(room));
    let mut messages = database::read::<Vec<Message>>(messages_database(room));
    if !users::verify(message.user.clone(), message.pass.clone()) {
        return;
    }
    messages.push(Message {
        user: message.user.clone(),
        body: message.body.clone(),
    });
    database::save(messages_database(room), messages);
}

#[get("/<room>/<count>")]
pub fn get_message(room: usize, count: usize) -> Json<Vec<Message>> {
    let mut messages = database::read::<Vec<Message>>(messages_database(room));
    let mut return_messages = vec![];
    let mut i = 0;
    while let Some(m) = messages.pop() {
        return_messages.push(m);
        i += 1;
        if i >= count.min(250) {
            break;
        }
    }
    Json(return_messages)
}

fn messages_database(room: usize) -> String {
    format!("/data/rooms/{}/messages.ron", room)
}
