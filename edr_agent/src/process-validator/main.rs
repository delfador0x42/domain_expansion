use std::process::Command;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Create the command
    let output = Command::new("sudo")
        .arg("ps")
        .arg("aux")
        .output()?;

    // Check if the command was successful
    if output.status.success() {
        // Convert the output bytes to string and print
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Command output:\n{}", stdout);
    } else {
        // If command failed, print the error
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed with error:\n{}", stderr);
    }

    Ok(())
}