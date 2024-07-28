// defined errors

use axum::response::{
    IntoResponse,
    Response,
};
use axum::body::Body;
use core::result::Result;

pub(crate) type Results<T> = Result<T,Exception>;

pub enum Exception {
    // TODO : implement for tracing
    ClientSideError404,
}


impl IntoResponse for Exception {
    // asusual implemantaion
    fn into_response(self) -> Response<Body> {
        match self {
            Self::ClientSideError404 => {
                Body::from("404".to_owned()).into_response()
            }
        }
    }
}


