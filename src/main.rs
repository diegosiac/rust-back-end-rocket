#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hola, mundo con Rocket!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
