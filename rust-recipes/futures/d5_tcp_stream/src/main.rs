use anyhow::{Context, Result};
use futures::stream::StreamExt;
use tcpstream::ReadStream;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .context("Failed to bind to address")?;

    loop {
        let (socket, _) = listener
            .accept()
            .await
            .context("Failed to accept connection")?;
        println!("Accepted connection from: {}", socket.peer_addr()?);

        tokio::spawn(async move {
            let mut read_stream = ReadStream::new(socket);
            while let Some(s) = read_stream.next().await {
                if s.is_empty() {
                    break;
                }
                println!("{}", s);
            }
        });
    }
}
