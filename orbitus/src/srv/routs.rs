use axum::response::IntoResponse;
use axum::Router;

use crate::err::Exception;


pub fn fallback_route() -> Router {
    Router::new()
        .fallback( || async {
            Exception::ClientSideError404
        })
}
