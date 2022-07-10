#![allow(dead_code, unreachable_code, unused_imports)]
#![feature(option_result_contains)]

fn answer() -> u8 {
    return 42;
}

use anyhow::Ok;
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
// use std::io::{self, BufRead};
// const PREFIX: &str = "found - ";
use anyhow::{anyhow, Context, Error, Result};
use std::io::{self, Write};

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

use grrs_cli_book_demo_deploy_test::{find_matches, PREFIX};

use log::{info, trace};

// use std::error::Error;
use std::fmt;
use std::{thread, time};

#[derive(Debug)]
struct CustomErr(String);

// std::io::Result<()>
// std::io::Result !== std::result::Result
// fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
// fn main() -> std::result::Result<(), CustomErr> {
// or, if there are multiple binaries, cargo expects them to be in src/bin/<name>.rs
fn main() -> anyhow::Result<()> {
    // env RUST_LOG=info cargo run --bin output-log
    // if file is output-log
    // for this case just
    // env RUST_LOG=info cargo run
    //
    // https://crates.io/crates/clap-verbosity-flag
    //
    env_logger::init();
    info!("info env log lvl, also visible inside trace");
    trace!("trace and all");
    // progress examples
    // https://github.com/console-rs/indicatif/tree/main/examples
    // let pb = indicatif::ProgressBar::new(100);
    // for i in 0..100 {
    //     // do the work here
    //     thread::sleep(time::Duration::from_millis(0));
    //     pb.println(format!("[+] finished #{}", i));
    //     pb.inc(1);
    // }
    // pb.finish_with_message("done");

    let args = Cli::parse();
    if args.pattern.is_empty() {
        // println!("the pattern is empty");
        return Err(anyhow!("the pattern is empty"));
    }

    let file_as_string = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    find_matches(&file_as_string, &args.pattern, &mut std::io::stdout())?;

    Ok(())
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

    // println!("file_as_string: {:?}", file_as_string);

    // note on printing performance
    // let stdout = io::stdout(); // get the global stdout entity
    // let mut handle = io::BufWriter::new(stdout.lock()); // optional: wrap that handle in a buffer
    // writeln!(handle, "{}", file_as_string)?;
    // // no need for `.map_err(|err| Error::new())`
    // handle.flush()?;
    // Ok(())

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_answer_validity() {
        assert_eq!(answer(), 42);
    }
}
