use axum::Router;
use axum::response::IntoResponse;
use axum::extract::Path;
use axum::routing::get;
use axum::http::header;
use tracing::{info, warn};
use crate::err::Exception;
use rust_embed::Embed;

use super::pages::main_page_handeler_get;

pub fn fallback_route() -> Router {
    Router::new()
        .fallback( || async {
            Exception::ClientSideError404
        })
}

pub async fn asset_loader(Path(path) : Path<String>) -> impl IntoResponse {
    info!("Asset loader: {}",path.clone());
    match Asset::get(&path.clone()) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            info!("Dispatch Type : {}",mime);
            ([(header::CONTENT_TYPE,mime.as_ref())],content.data).into_response()
        },
        None => {
            warn!("invalid resource requested");
            ("ASSET NOT FOUND").into_response()
        }
    }
}

pub fn spa_router() -> Router {
    Router::new()
        .route("/", get(main_page_handeler_get))
        .merge(fallback_route())
}

pub fn asset_router() -> Router {
    Router::new()
        .route_service("/__assets/*path", get(asset_loader))
}

#[derive(Embed)]
#[folder = "src/build/__assets/"]
struct Asset;


