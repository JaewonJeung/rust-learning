mod connection;
mod frame;
mod parse;

use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpStream;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

/// Handle a single connection: read up to 3 bytes then echo them back.
/// Returns the number of bytes echoed.
pub async fn handle_connection(mut socket: TcpStream) -> io::Result<usize> {
    let mut buf = [0u8; 3];
    let n = loop {
        socket.readable().await?;
        match socket.try_read(&mut buf) {
            Ok(0) => return Ok(0), // connection closed
            Ok(n) => break n,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
            Err(e) => return Err(e),
        }
    };
    socket.write_all(&buf[..n]).await?;
    socket.shutdown().await?;
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn echoes_three_bytes_or_less() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        // Spawn server accept once
        let server = tokio::spawn(async move {
            let (socket, _addr) = listener.accept().await.unwrap();
            handle_connection(socket).await.unwrap()
        });

        // Client
        let mut client = TcpStream::connect(addr).await.unwrap();
        client.write_all(b"abc").await.unwrap();

        let mut out = Vec::with_capacity(3);
        let mut tmp = [0u8; 3];
        let n = client.read(&mut tmp).await.unwrap();
        out.extend_from_slice(&tmp[..n]);

        let echoed = server.await.unwrap();
        assert_eq!(echoed, 3);
        assert_eq!(&out, b"abc");
    }

    #[tokio::test]
    async fn many_connections() {
        const N: usize = 25; // keep test fast
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        // server task
        let server = tokio::spawn(async move {
            for _ in 0..N {
                let (socket, _addr) = listener.accept().await.unwrap();
                tokio::spawn(async move {
                    let _ = handle_connection(socket).await;
                });
            }
        });

        // clients
        for i in 0..N {
            let mut c = TcpStream::connect(addr).await.unwrap();
            let msg = format!("{:03}", i); // 3 bytes always
            c.write_all(msg.as_bytes()).await.unwrap();
            let mut buf = [0u8; 3];
            let n = c.read(&mut buf).await.unwrap();
            assert_eq!(n, 3);
            assert_eq!(&buf, msg.as_bytes());
        }

        server.await.unwrap();
    }
}
