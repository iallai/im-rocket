use rocket_contrib::json::JsonValue;

#[get("/health")]
fn health() -> JsonValue {
    json!({
        "code":200
    })
}
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/", routes![index, health])
}
