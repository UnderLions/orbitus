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



#[derive(Debug,Subcommand)]
pub enum CliCommands{
    Start(StartCommandOptions)
}


#[derive(Debug, Args)]
pub struct StartCommandOptions {
    #[arg(long="host",help="specify ip address and port", default_value="127.0.0.1:8080")]
    pub host: SocketAddr
}

// #[allow(non_snake_case)]
pub fn init() {
    let cli = Cli::parse();

    match cli.cmd {
        CliCommands::Start(start_command_options) => {
            srv::start_serv(start_command_options);
        }
        // _ => {}
    }
}
