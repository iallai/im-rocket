use rocket_contrib::json::JsonValue;

#[get("/health")]
fn health() -> JsonValue {
    json!({
        "code":200,
        "message":"Hey ~ Api Goes Well 😁"
    })
}
#[get("/")]
fn index() -> JsonValue {
    json!({
        "code":200,
        "message":"IM Rocket 🚀"
    })
}
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/", routes![index, health])
}
