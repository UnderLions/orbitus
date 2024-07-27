use std::usize;

use crate::cli::StartCommandOptions;
use axum::{self, routing::future::RouteFuture};
use tokio::runtime::{Builder, Runtime};
use axum::Router;
use axum::response::IntoResponse;
use axum::routing::get;


mod routs;

fn router() -> Router<> {
    Router::new()
        .route("/", get(|| async { "<p>404</p>".into_response() }))
}

fn multi_thread_runtime(n_threads: usize) -> Result<Runtime, std::io::Error> {
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
