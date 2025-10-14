use bytes::Bytes;
use redis_client::dlog; // macro import
use redis_client::{connection::Connection, frame::Frame};
use std::env;
use tokio::net::TcpStream;

fn usage() -> &'static str {
    "Usage:\n  redis-client [addr] set <key> <value>\n  redis-client [addr] get <key>\nExamples:\n  redis-client set foo bar\n  redis-client 127.0.0.1:6379 get foo\nIf addr omitted, defaults to 127.0.0.1:6379"
}

fn build_bulk<S: AsRef<[u8]>>(s: S) -> Frame {
    Frame::Bulk(Bytes::from(s.as_ref().to_vec()))
}

fn build_set_frame(key: &str, value: &str) -> Frame {
    let mut parts = Vec::with_capacity(3);
    parts.push(build_bulk("SET"));
    parts.push(build_bulk(key));
    parts.push(build_bulk(value));
    Frame::Array(parts)
}

fn build_get_frame(key: &str) -> Frame {
    let mut parts = Vec::with_capacity(2);
    parts.push(build_bulk("GET"));
    parts.push(build_bulk(key));
    Frame::Array(parts)
}

/*
cargo run -- set foo bar
cargo run -- get foo
cargo run -- 127.0.0.1:6379 set another value
cargo run -- 127.0.0.1:6379 get another
*/

#[tokio::main]
async fn main() -> redis_client::Result<()> {
    dlog!("client start");
    let mut args: Vec<String> = env::args().skip(1).collect();
    dlog!("raw args: {:?}", args);

    let default_addr = "127.0.0.1:6379".to_string();
    let mut addr = default_addr.clone();

    // Detect if first argument looks like host:port
    if let Some(first) = args.first() {
        if first.contains(':') {
            addr = args.remove(0);
        }
    }

    if args.is_empty() {
        eprintln!("{}", usage());
        return Ok(());
    }

    let cmd = args.remove(0).to_lowercase();

    dlog!("connecting to {}", addr);
    let stream = TcpStream::connect(&addr).await?;
    dlog!("connected");
    let mut conn = Connection::new(stream);

    match cmd.as_str() {
        "set" => {
            if args.len() != 2 {
                eprintln!("SET requires <key> <value>\n{}", usage());
                return Ok(());
            }
            let key = &args[0];
            let value = &args[1];
            dlog!("building SET key='{}' value='{}'", key, value);
            let frame = build_set_frame(key, value);
            dlog!("sending frame: {:?}", frame);
            conn.write_frame(&frame).await?;
            dlog!("frame sent - awaiting response");
            if let Some(resp) = conn.read_frame().await? {
                dlog!("received frame: {:?}", resp);
                match resp {
                    Frame::Simple(s) => println!("OK: {}", s),
                    Frame::Error(e) => eprintln!("Error: {}", e),
                    other => println!("Response: {}", other),
                }
            }
        }
        "get" => {
            if args.len() != 1 {
                eprintln!("GET requires <key>\n{}", usage());
                return Ok(());
            }
            let key = &args[0];
            dlog!("building GET key='{}'", key);
            let frame = build_get_frame(key);
            dlog!("sending frame: {:?}", frame);
            conn.write_frame(&frame).await?;
            dlog!("frame sent - awaiting response");
            if let Some(resp) = conn.read_frame().await? {
                dlog!("received frame: {:?}", resp);
                match resp {
                    Frame::Bulk(bytes) => match std::str::from_utf8(&bytes) {
                        Ok(s) => println!("Value: {}", s),
                        Err(_) => println!("(binary {:?})", bytes),
                    },
                    Frame::Null => println!("(nil)"),
                    Frame::Error(e) => eprintln!("Error: {}", e),
                    other => println!("Response: {}", other),
                }
            }
        }
        other => {
            eprintln!("Unknown command '{}'.\n{}", other, usage());
        }
    }

    dlog!("client done");
    Ok(())
}
