use tokio::net::TcpListener;

use redis::handle_connection;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (socket, _addr) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                eprintln!("connection error: {e}");
            }
        });
    }
}
