use anyhow::{Error, Result};

pub const PREFIX: &str = "found - ";

// business logic of the cli
pub fn find_matches(
    content: &str,
    pattern: &str,
    mut writer: impl std::io::Write,
) -> Result<(), Error> {
    for line in content.lines() {
        if line.contains(pattern) {
            // println! works the same as writeln! but always uses standard output
            // correct err handling:
            // with PREFIX
            writeln!(writer, "{}{}", PREFIX, line)?;
            // without:
            // writeln!(writer, "{}", line)?;
            // bad:
            // just notify: error in write op without returning Result
            // let res = writeln!(writer, "found - {}", line);
            // match res {
            //     Ok(_) => {
            //         println!("wrote to `writer` succefully")
            //     }
            //     Err(_) => {
            //         println!("failed to write to `writer` succefully")
            //     }
            // }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    // you have to import everything in this module
    use super::*;

    #[test]
    fn find_match() {
        let mut res = Vec::new();
        let ret = find_matches("hi\nlol and ok\nlulz", "lol", &mut res);
        // The b prefix makes this a byte string literal
        // so its type is going to be &[u8] instead of &str
        // certainly need to handle this instead of unwrapping

        // write prefix
        // writeln!(res, "{}", PREFIX);
        let mut to_be_compared: String = String::new();

        let shoud_find_string = "lol and ok\n";
        to_be_compared.push_str(PREFIX);
        to_be_compared.push_str(&shoud_find_string);
        println!("to be comp: {}", to_be_compared);
        assert_eq!(ret.unwrap(), ());
        // first iteration without prefix
        // assert_eq!(res, b"lol and ok\n");
        // without:
        assert_eq!(res, to_be_compared.as_bytes());
    }
}
