use std::env;
use std::os::unix::net::UnixStream;
use std::io::{self, Read, Write};
use std::thread;

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect("/tmp/game.sock")?;

    println!("Connected to game server!");

    let mut buffer = [0u8; 1024];
    
    // Get players name
    let player_name = match env::args().nth(1) {
        Some(name) => name,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Usage: cargo run --bin client -- <player-name>",
            ));
        }
    };

    // Send the name to server
    stream.write_all(player_name.as_bytes())?;

    // Spawn a thread to listen to the server
    let listerner_stream = stream.try_clone()?;

    let server_handler = thread::spawn( move || {
        server_listener(listerner_stream);
    });

    loop {};

    Ok(())
}

fn server_listener(mut stream: UnixStream) {
    let mut buffer = [0u8; 1024];

    loop {
        let n = stream.read(&mut buffer).unwrap();
        if n > 0 {
            println!("N is: {n}");
            println!(
                "{}",
                String::from_utf8_lossy(&buffer[..n])
            );
        }
    };
}