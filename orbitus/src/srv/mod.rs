use std::usize;

use crate::cli::StartCommandOptions;
use axum;
use tokio::runtime::{Builder, Runtime};


fn multi_thread_runtime(n_threads: usize) -> Result<Runtime, std::io::Error> {
    Builder::new_multi_thread()
        .worker_threads(n_threads)
        .enable_io()
        .thread_name("orbitus_rt")
        .build()
}


pub fn start_serv(StartCommandOptions { host ,..}: StartCommandOptions) {
    println!("host at {:?}", host);
}