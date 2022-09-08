#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rusqlite::Connection;
use serde::Serialize;

#[launch]
fn rocket() -> _ {
    let db_connection = Connection::open("data.sqlite").unwrap();
    db_connection
        .execute(
            "create table if not exists messages (
                id integer primary key,
                user varchar(64) not null,
                body varchar(64) not null
            );",
            rusqlite::NO_PARAMS,
        )
        .unwrap();

    rocket::build().mount("/", routes![index, get, send])
}

#[get("/")]
fn index() -> String {
    "Welcome to my api!".to_string()
}

#[get("/message")]
fn get() -> Result<Json<Vec<Message>>, String> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => return Err("Failed to connect to database".into()),
    };

    let mut statement = match db_connection.prepare("select id, user, body from messages;") {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to select messages from database".into()),
    };

    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(Message {
            id: row.get(0)?,
            user: row.get(1)?,
            body: row.get(2)?,
        })
    });

    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<Message>> = rows.collect();
            match collection {
                Ok(items) => Ok(Json(items)),
                Err(_) => return Err("Failed to collect items".into()),
            }
        },
        Err(_) => return Err("Failed to fetch messages".into()),
    }
}

#[post("/message", format = "json", data = "<item>")]
fn send(item: Json<String>) -> Result<Json<StatusMessage>, String> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    let mut statement =
        match db_connection.prepare(&format!("insert into messages (id, user, body) values (null, {}, {});", "usererr", "bodyodyody")) {
            Ok(statement) => statement,
            Err(e) => return Err(e.to_string()),
        };
    let results = statement.execute(&[&item.0]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows inserted!", rows_affected),
        })),
        Err(e) => Err(e.to_string()),
    }
}

#[derive(Serialize)]
struct Message {
    id: i64,
    user: String,
    body: String,
}

#[derive(Serialize)]
struct StatusMessage {
    message: String,
}
