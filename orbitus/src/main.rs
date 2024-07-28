// TODO : development lints here some times unused and/or dead_code allowed
// #![allow(unused)]
//
//
//
// --------------------------------------------------------------------------


// LINTS
//
// #![forbid(unsafe_code)]
// it is not necessary ,because already done in Cargo.toml
// so no unsafe_code here!
// 
// TODO: Doc it
// #![warn(missing_docs)]



// include all modules in this crate
mod cli;
mod srv;
mod err;

fn main() {
    // call cli initializer
    cli::init();
}
