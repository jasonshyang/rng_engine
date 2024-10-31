use clap::Parser;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Parser)]
#[clap(author, version, about, long_about = "CLI to interact with the server")]
struct Args {
    #[clap(short, long)]
    id: u64,

    #[clap(short, long, default_value = "127.0.0.1:3000")]
    server: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut stream = TcpStream::connect(args.server).await?;
    stream.write_all(args.id.to_string().as_bytes()).await?;

    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;
    let response = String::from_utf8_lossy(&buf[..n]);

    println!("Response: {}", response);

    Ok(())
}
