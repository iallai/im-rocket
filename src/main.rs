#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod organization {
    #[get("/organizations/list")]
    pub fn list() -> &'static str {
        "organizations list"
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[get("/buzz")]
fn fizz_buzz() -> &'static str {
    "buzz!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, organization::list, fizz_buzz])
        .launch();
}
