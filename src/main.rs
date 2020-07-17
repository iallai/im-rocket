#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

mod db;
mod resources;
use rocket::Request;
use rocket_contrib::json::JsonValue;

#[catch(404)]
fn not_found(req: &Request) -> JsonValue {
    let message = format!("'{}' was not found", req.uri());
    json!({
        "code":404,
        "message":message
    })
}
fn main() {
    let mut rocket = rocket::ignite().manage(db::establish_connection);
    rocket = resources::health::mount(rocket);
    rocket = resources::organization::mount(rocket);
    rocket.register(catchers![not_found]).launch();
}
