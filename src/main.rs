#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
mod health;
mod organization;
fn main() {
    let mut rocket = rocket::ignite();
    rocket = health::mount(rocket);
    rocket = organization::mount(rocket);
    rocket.launch();
}
