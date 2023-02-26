use std::vec;

use rocket::serde::{json::Json, Serialize};

use crate::models::Article;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ArticleListResponse {
    items: Vec<Article>,
}

#[get("/")]
pub fn get_list_handler() -> Json<ArticleListResponse> {
    let response = ArticleListResponse { items: vec![] };

    Json(response)
}
