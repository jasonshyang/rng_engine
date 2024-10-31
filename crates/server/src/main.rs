use std::sync::Arc;
use tokio::sync::Mutex;

use tokio::{
    net::TcpListener,
    io::{AsyncReadExt, AsyncWriteExt},
};

use core::rng_engine::RngEngine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rng_engine = Arc::new(Mutex::new(RngEngine::new(42))); // Seed should be random in production
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000)); // Listen on localhost:3000, should be configurable
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on http://{}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;
        let rng_engine = rng_engine.clone();

        tokio::spawn(async move {
            let mut buf = [0; 1024]; // Buffer size should be configurable

            let n = match socket.read(&mut buf).await {
                Ok(n) if n > 0 => n,
                Ok(_) => {
                    eprintln!("failed to read from socket; err = zero bytes read");
                    return;
                }
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };
    
            let request = String::from_utf8_lossy(&buf[..n]);
            print!("Request: {}", request);
            match request.trim().parse::<u64>() {
                Ok(id) => {
                    let mut rng_engine = rng_engine.lock().await;
                    let rng_result = rng_engine.generate(id);
                    let response = format!("Random number: {}\n", rng_result);

                    if let Err(e) = socket.write_all(response.as_bytes()).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                    }
                }
                Err(e) => {
                    eprintln!("failed to parse request; err = {:?}", e);
                    let response = format!("Invalid request: {}\n", request);
                    if let Err(e) = socket.write_all(response.as_bytes()).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                    }
                }
            }
        });
    }
}
