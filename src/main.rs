use crate::schema::todo;
use diesel::prelude::*;
use diesel::{Insertable, PgConnection, Queryable};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, launch, post, put, routes};
use rocket_sync_db_pools::database;

use dotenv::dotenv;
// use rocket::figment::{providers::Env, providers::Toml, Figment};
// use std::env;

#[database("myDb")]
struct DbConn(PgConnection);

#[macro_use]
extern crate diesel;

mod schema;

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

#[get("/")]
async fn get_todo(conn: DbConn) -> Json<Vec<Todo>> {
    let todos = conn
        .run(|c| todo::table.order(todo::columns::id.desc()).load::<Todo>(c))
        .await
        .unwrap();

    Json(todos)
}

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

#[put("/<id>")]
async fn check_todo(conn: DbConn, id: i32) -> Json<Todo> {
    let target = todo::table.filter(todo::columns::id.eq(id));

    let my_result = conn
        .run(move |db| {
            diesel::update(target)
                .set(todo::columns::checked.eq(true))
                .get_result::<Todo>(db)
        })
        .await
        .unwrap();

    Json(my_result)
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![get_todo, create_todo, check_todo])

    /*Tried to implement env PORT for listening port */

    // let figment = Figment::from(rocket::Config::default()).merge(("port", 8089));
    // let my_port = env!("PORT", "8080");
    // let figment = Figment::from(rocket::Config::default())
    // .merge(Env::prefixed("ROCKET_"))
    // .merge(Toml::from("config.toml"))
    // .merge(("port", env::var("PORT").unwrap()))
    // .merge(("ROCKET_DATABASES", env::var("ROCKET_DATABASES").unwrap()));

    // let figment = Figment::from(rocket::Config::default()).merge(Env::prefixed("PORT").global());

    // rocket::custom(figment)
    //     .mount("/", routes![get_todo, create_todo])
    //     .attach(DbConn::fairing())
}
