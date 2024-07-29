use axum::{http::header, response::IntoResponse};

fn spa_page_1() -> &'static str {
    include_str!("../../src/build/index.html")
}


pub async fn main_page_handeler_get() -> impl IntoResponse {
    ([(header::CONTENT_TYPE,"text/html")],spa_page_1()).into_response()
}
