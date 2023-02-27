use std::error::Error;

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[macro_use]
extern crate rocket;

mod handlers;
mod models;

// #[launch]
// fn rocket() -> Rocket<Build> {
//     let cors = CorsOptions::default()
//         .allowed_origins(AllowedOrigins::all())
//         .allowed_methods(
//             vec![Method::Get, Method::Post, Method::Patch]
//                 .into_iter()
//                 .map(From::from)
//                 .collect(),
//         )
//         .allow_credentials(true)
//         .to_cors();

//     rocket::build()
//         .mount("/articles", routes![handlers::article::get_list_handler])
//         .manage(cors)
// }

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let allowed_origins = AllowedOrigins::some_exact(&["http://127.0.0.1:5173"]);

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Put,
            Method::Patch,
            Method::Delete,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    let _ = rocket::build()
        .mount("/articles", routes![handlers::article::get_list_handler])
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
