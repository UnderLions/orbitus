use axum::{http::header, response::IntoResponse};
use axum::Json;
use axum::extract::{self, State};
use serde::Deserialize;
use crate::api::{ AtomicDB , UserAction , CRUD};

#[derive(Deserialize, Debug)]
struct UserEntry {
    username: String,
    password: String
}


fn spa_page_root() -> &'static str {
    include_str!("../../src/build/index.html")
}


pub async fn main_page_handler_get() -> impl IntoResponse {
    ([(header::CONTENT_TYPE,"text/html")],spa_page_root()).into_response()
}

pub async fn root_auth_create_post(State(state): State<AtomicDB> , Json(payload): Json<UserEntry>) -> impl IntoResponse {
    println!("{:?}", payload);
    match state.create(payload.username.clone(), payload.username.clone()) {
        Some(_) => "created".into_response(),
        None => "Failed".into_response()
    }
}

pub async fn root_auth_login_post(State(state): State<AtomicDB>,Json(payload): Json<UserEntry>) -> impl IntoResponse {
    println!("{:?}", payload);
    if state.verify(&payload.username, &payload.password) {
        "on".into_response()
    } else { "failed".into_response() }
}
