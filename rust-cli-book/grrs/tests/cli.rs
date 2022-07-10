use assert_cmd::prelude::*; // Add methods on commands
use grrs::PREFIX;
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}
use assert_fs::prelude::*;
// output when running cargo run -- test sample.txt
// found - A test
// found - Another test
#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    let output_hard = "found - A test\nfound - Another test\n";
    let formated = format!("{PREFIX}A test\n{PREFIX}Another test\n");

    assert_eq!(output_hard, formated);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(formated));

    Ok(())
}
// challenge:
// clear && cargo run -- '' sample.txt
// output all the lines, instead require not empty string
// "the pattern is empty"
#[test]
fn you_should_provide_nonempty_pettern() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("").arg(file.path());

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains(""))
        .stderr(predicate::str::contains("the pattern is empty"));

    Ok(())
}
