use std::process::{Command, ExitStatus};

pub fn ls() -> ExitStatus {
    let mut ls = Command::new("ls");

    // .status() already print the result
    return ls.status().expect("process failed to execute");
}
