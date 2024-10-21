// TODO : development lints here some times unused and/or dead_code allowed
// #![allow(unused)]
//
//
//
// --------------------------------------------------------------------------


// LINTS
//
// #![forbid(unsafe_code)]
// 
// TODO: Doc it
// #![warn(missing_docs)]



// include all modules in this crate
mod cli;
mod srv;
mod err;
mod api;

fn main() {
    // call cli initializer
    cli::init();
}
