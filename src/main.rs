#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod resources;
use rocket::Request;
use rocket_contrib::json::JsonValue;

#[catch(404)]
fn not_found(req: &Request) -> JsonValue {
    let message = format!("'{}' 404 not found", req.uri());
    json!({
        "code":404,
        "message":message
    })
}
fn main() {
    let mut rocket = rocket::ignite();
    rocket = resources::health::mount(rocket);
    rocket = resources::organization::mount(rocket);
    rocket.register(catchers![not_found]).launch();
}
