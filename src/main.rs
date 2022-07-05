#[macro_use]
extern crate rocket;

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
    rocket::build().mount("/", routes![index, foo])
}
