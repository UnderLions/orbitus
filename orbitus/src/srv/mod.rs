use std::sync::Arc;

use crate::{api::AtomicDB, cli::StartCommandOptions};
use routs::asset_router;
use tokio::runtime::{Builder, Runtime};
use axum::{routing::{get, post}, Router};

// include all submodules under src::xxxx;
mod routs;
mod pages;

// TODO : remove this sh*t
fn router() -> Router<()> { 
    Router::new()
        .route("/", get(pages::main_page_handler_get))
        .route("/auth/creat", post(pages::root_auth_create_post))
        .route("/auth/login", post(pages::root_auth_login_post))
        .with_state(routs::model())
        .merge(asset_router())
}


fn multi_thread_runtime(n_threads: usize) -> Result<Runtime, std::io::Error> {
    // it simply returns runtime builder object
    //
    // TODO : replace print with tracing [low priority]
    Builder::new_multi_thread()
        .worker_threads(n_threads)
        .enable_all()
        .thread_name("orbitus_rt")
        .on_thread_stop(|| {
            println!("stopping thread");
        })
        .on_thread_start(|| {
            println!("starting thread");
        })
        .build()
}

// TODO : remove unwrap and write actual safe rust
pub fn start_serv(StartCommandOptions { host, .. }: StartCommandOptions) {
    println!("host at {:?}", host);
    let runnerer = multi_thread_runtime(2).unwrap();
    let _block_on = runnerer.block_on(
        async {
            let tcp = tokio::net::TcpListener::bind(host).await.unwrap();
            let routes = router();
            let _ = axum::serve(tcp, routes).await;

        }
    );
}
