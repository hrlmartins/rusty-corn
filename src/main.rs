#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate chrono;

extern crate reqwest;

extern crate futures;


mod routes;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, routes::movies::list_movies_in_display,])
        .launch();
}
