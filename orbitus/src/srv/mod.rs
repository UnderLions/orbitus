use std::usize;

use crate::cli::StartCommandOptions;
use tokio::runtime::{Builder, Runtime};
use axum::Router;
use axum::response::IntoResponse;
use axum::routing::get;

// include all submodules under src::xxxx;
mod routs;
mod pages;

// TODO : remove this sh*t
fn router() -> Router<> {
    Router::new()
        .route("/", get(|| async { "<p>404</p>".into_response() }))
        .merge(routs::fallback_route())
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
