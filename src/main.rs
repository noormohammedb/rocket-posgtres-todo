use crate::schema::todo;
use diesel::prelude::*;
use diesel::{Insertable, PgConnection, Queryable};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, launch, post, routes};
use rocket_sync_db_pools::database;

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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![get_todo, create_todo])
}
