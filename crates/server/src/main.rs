use std::{
    env,
    sync::Arc,
};
use tokio::{
    net::TcpListener,
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Mutex
};
use core::{
    db::Db,
    rng_engine::RngEngine
};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = Arc::new(Mutex::new(Db::new(&database_url).await?));
    db.lock().await.init().await?;

    let rng_engine = Arc::new(Mutex::new(RngEngine::new(42, db.clone()))); // Seed should be random in production

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
            match request.trim().parse::<i64>() {
                Ok(player_id) => {
                    let mut rng_engine = rng_engine.lock().await;
                    match rng_engine.generate(player_id).await {
                        Ok(rng_result) => {
                            let response = format!("{}\n", rng_result);
                            if let Err(e) = socket.write_all(response.as_bytes()).await {
                                eprintln!("failed to write to socket; err = {:?}", e);
                            }
                        }
                        Err(e) => {
                            eprintln!("failed to generate RNG result; err = {:?}", e);
                            let response = "Failed to generate RNG result\n";
                            if let Err(e) = socket.write_all(response.as_bytes()).await {
                                eprintln!("failed to write to socket; err = {:?}", e);
                            }
                        }
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
