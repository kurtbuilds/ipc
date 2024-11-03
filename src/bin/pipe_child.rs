use std::io::{self, Read, Write};


fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle = stdin.lock();
    let mut out_handle = stdout.lock();

    let mut buffer = [0u8; 1024];

    loop {
        // Read exactly 1024 bytes from stdin
        match handle.read_exact(&mut buffer) {
            Ok(_) => {
                // Write the same data back to stdout after reading the full message
                out_handle.write_all(&buffer)?;
                out_handle.flush()?;
            },
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                // Exit loop if we encounter EOF before 1024 bytes
                break;
            },
            Err(e) => return Err(e), // Propagate any other error
        }
    }

    Ok(())
}