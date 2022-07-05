#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

mod schema;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<foo>")]
fn foo(foo: String) -> String {
    format!("Hello, {}!", foo)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, foo])
        .mount("/todo", routes![])
}
