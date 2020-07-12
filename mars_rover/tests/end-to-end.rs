use std::io::prelude::*;
use std::process::{Command, Stdio};

#[test]
fn test() -> std::io::Result<()> {
    let mut child = Command::new("./target/debug/mars_rover")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let child_stdin = child.stdin.as_mut().unwrap();
    child_stdin.write_all(b"3 3\n")?;
    child_stdin.write_all(b"0 0 N\n")?;
    child_stdin.write_all(b"F\n")?;
    child_stdin.write_all(b"\n")?;

    let output = child.wait_with_output()?;

    assert_eq!(std::str::from_utf8(&output.stdout).unwrap(), "0 1 N\n");

    Ok(())
}

#[test]
fn move_north_twice() -> std::io::Result<()> {
    let mut child = Command::new("./target/debug/mars_rover")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let child_stdin = child.stdin.as_mut().unwrap();
    child_stdin.write_all(b"3 3\n")?;
    child_stdin.write_all(b"0 0 N\n")?;
    child_stdin.write_all(b"F\n")?;
    child_stdin.write_all(b"F\n")?;
    child_stdin.write_all(b"\n")?;

    let output = child.wait_with_output()?;

    assert_eq!(std::str::from_utf8(&output.stdout).unwrap(), "0 2 N\n");

    Ok(())
}

#[test]
fn move_north_then_back_south() -> std::io::Result<()> {
    let mut child = Command::new("./target/debug/mars_rover")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let child_stdin = child.stdin.as_mut().unwrap();
    child_stdin.write_all(b"3 3\n")?;
    child_stdin.write_all(b"0 0 N\n")?;
    child_stdin.write_all(b"F\n")?;
    child_stdin.write_all(b"L\n")?;
    child_stdin.write_all(b"L\n")?;
    child_stdin.write_all(b"F\n")?;
    child_stdin.write_all(b"L\n")?;
    child_stdin.write_all(b"L\n")?;
    child_stdin.write_all(b"\n")?;

    let output = child.wait_with_output()?;

    assert_eq!(std::str::from_utf8(&output.stdout).unwrap(), "0 0 N\n");

    Ok(())
}
