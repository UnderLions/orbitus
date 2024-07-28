use crate::srv;
use clap::{
    command, Args, Parser, Subcommand
};
use std::net::SocketAddr;


#[derive(Debug ,Parser)]
#[command(about,long_about)]
struct Cli {

    #[command(subcommand)]
    cmd: CliCommands
}


// TODO : add environment variables to override default option
#[derive(Debug,Subcommand)]
pub enum CliCommands{
    // subcommand to start listening
    // <bin> start --options
    Start(StartCommandOptions)
}


#[derive(Debug, Args)]
pub struct StartCommandOptions {

    // tcp socket address for initiate
    #[arg(long="host",help="specify ip address and port", default_value="127.0.0.1:8080")]
    pub host: SocketAddr
    // TODO : add n_threads arg for async runtime
}


pub fn init() {
    // initialiate cli parser
    let cli = Cli::parse();

    match cli.cmd {
        CliCommands::Start(start_command_options) => {
            // start service have tokio runtime bulder and axum service
            // axum router on srv::route module
            srv::start_serv(start_command_options);
        }
        // no default_value currently
        // _ => {}
    }
}
