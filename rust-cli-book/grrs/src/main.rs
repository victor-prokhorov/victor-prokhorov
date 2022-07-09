#![allow(dead_code)]

use std::env;

#[derive(Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

use clap::Parser;

#[derive(Parser, Debug)]
struct ClapCli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let clap_args = ClapCli::parse();
    println!("{:?}", clap_args);

    // print collected arguments
    let collected_args: Vec<String> = env::args().collect();
    println!("collected arguments: {:?}", collected_args);

    // print argument on each line
    for argument in env::args() {
        println!("argument: {argument}");
    }

    let pattern = std::env::args()
        .nth(1)
        .expect("pattern argument should be given");

    let path = std::env::args()
        .nth(2)
        .expect("path argument should be given");

    let args = Cli {
        pattern,
        path: std::path::PathBuf::from(path),
    };

    println!("{:?}", args);
    println!("end");
}
