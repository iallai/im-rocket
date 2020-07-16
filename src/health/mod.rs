#[get("/health")]
fn health() -> &'static str {
    "ping pong~"
}
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/", routes![index, health])
}
