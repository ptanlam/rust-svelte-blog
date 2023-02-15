use rocket::http::Status;

struct Article {
    name: String,
    content: String,
}

#[get("/")]
pub fn list() -> Article {
    (Status::Ok, "ok")
}
