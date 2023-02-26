use rocket::{Build, Rocket};

#[macro_use]
extern crate rocket;

mod handlers;
mod models;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/articles", routes![handlers::article::get_list_handler])
}
