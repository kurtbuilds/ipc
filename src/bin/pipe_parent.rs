use std::io::{self, Write, Read};
use std::process::{Command, Stdio};
use rand::Rng;

fn main() -> io::Result<()> {
    // Generate 1KB of random data
    let mut data = [0u8; 1024];
    rand::thread_rng().fill(&mut data);

    // Spawn the child process
    let mut child = Command::new("./pipe_child") // Replace with actual path
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut child_stdin = child.stdin.as_mut().expect("Failed to open child stdin");
    let mut child_stdout = child.stdout.as_mut().expect("Failed to open child stdout");

    let mut response = [0u8; 1024];
    for _ in 0..10 {
        // Write data to the child's stdin
        child_stdin.write_all(&data)?;

        // Read back the response from child's stdout
        child_stdout.read_exact(&mut response)?;

        // Assert the received data matches what was sent
        assert_eq!(data, response, "Data mismatch!");
    }
    // Ensure child process has exited
    // child.wait()?;
    Ok(())
}