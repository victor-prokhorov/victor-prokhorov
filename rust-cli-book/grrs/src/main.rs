#![allow(dead_code, unreachable_code, unused_imports)]
#![feature(option_result_contains)]

use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
// use std::io::{self, BufRead};
use anyhow::{Context, Result};

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}
// use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct CustomErr(String);

// std::io::Result<()>
// std::io::Result !== std::result::Result
// fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
// fn main() -> std::result::Result<(), CustomErr> {
fn main() -> Result<()> {
    let _path = "./text.txt";

    let args = Cli::parse();
    let file_as_string = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    // the version with CustomErr
    // let file_as_string = std::fs::read_to_string(&args.path).map_err(|err| {
    //     return CustomErr(format!(
    //         "Error while reading {}: {}",
    //         &args.path.display(),
    //         err
    //     ));
    // })?;

    // without ?
    // let content = match file_as_string {
    //     Ok(_) => {
    //         println!("file was found");
    //     }
    //     Err(error) => {
    //         // panic!("panic");
    //         let err = Err(error.into());
    //         return err;
    //     }
    // };
    // println!("content: {:?}", content);
    println!("file_as_string: {:?}", file_as_string);

    return Ok(());

    // for line in file_as_string?.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{line} was found inside {:?}", &args.path);
    //     }
    // }

    // let f = File::open(&args.path)?;
    // let reader = BufReader::new(f);

    // for (i, line) in reader.lines().enumerate() {
    //     println!("{}: {:?}", i + 1, &line);
    //     if line.contains(&args.pattern) {
    //         println!(
    //             "{:?} was found inside {:?} at line number {}",
    //             line?,
    //             &args.path,
    //             i + 1
    //         );
    //         break;
    //     }
    // }
    // TODO: notify that nothing was found
}
