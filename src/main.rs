use crate::schema::todo;
// use diesel::sql_types::Json;
use diesel::prelude::*;
use diesel::{Insertable, PgConnection, Queryable};
use rocket::serde::json::Json;
use rocket_sync_db_pools::database;
// use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, launch, post, routes};
// use rocket_contrib::databases::{database, diesel::PgConnection};
// use serde_derive::{json::Json, Deserialize, Serialize};
// use serde::{json::Json, Deserialize, Serializer};
// #[macro_use]
// extern crate rocket;

#[database("myDb")]
struct DbConn(PgConnection);

#[macro_use]
extern crate diesel;

mod schema;

// #[serde(crate = "rocket::serde")]
#[derive(Debug, Queryable, Serialize)]
struct Todo {
    id: i32,
    title: String,
    checked: bool,
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "todo"]
struct NewTodo {
    title: String,
}

/*
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<foo>")]
fn foo(foo: String) -> String {
    format!("Hello, {}!", foo)
}

*/

#[post("/todo", data = "<new_todo>")]
async fn create_todo(conn: DbConn, new_todo: Json<NewTodo>) -> Json<Todo> {
    let my_result = conn
        .run(move |c| {
            diesel::insert_into(todo::table)
                .values(&new_todo.0)
                .get_result(c)
        })
        .await
        .unwrap();
    Json(my_result)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![create_todo])
}
