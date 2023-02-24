use rocket::{
    http::Status,
    serde::{json::Json, Serialize},
};

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Article {
    name: String,
    content: String,

    #[serde(rename = "createdAt")]
    created_at: String,
}

impl Article {
    fn new(name: String, content: String, created_at: String) -> Self {
        Article {
            name,
            content,
            created_at,
        }
    }
}

#[derive(FromForm)]
pub struct ArticleListRequest {
    page_index: Option<usize>,
    page_size: Option<usize>,
}

#[get("/?<request..>")]
pub fn list(request: ArticleListRequest) -> Result<Json<Vec<Article>>, Status> {
    let articles: Vec<Article> = vec![
        Article::new(
            "name1".to_string(),
            "content1".to_string(),
            "yesterday".to_string(),
        ),
        Article::new(
            "name2".to_string(),
            "content2".to_string(),
            "today".to_string(),
        ),
    ];

    let page_size = request.page_size.unwrap_or(10);
    let skip = (request.page_index.unwrap_or(1) - 1) * page_size;

    let article_paged_list = articles.into_iter().skip(skip).take(page_size).collect();

    Ok(Json(article_paged_list))
}
