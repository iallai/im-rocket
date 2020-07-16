#[get("/list")]
fn list() -> &'static str {
    "organizations list"
}
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/organizations", routes![list])
}
