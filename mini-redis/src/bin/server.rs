use std::io;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

mod client;

#[tokio::main]
async fn main() -> io::Result<()> {
    start_server().await
}

async fn start_server() -> io::Result<()> {
    let listener = TcpListener::bind(config::BINDING_ADDRESS).await?;
    println!("[Server] started...");
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("[Server] accepted a client: [{}]", addr);
        process_socket(socket).await?;
    }
}

async fn process_socket(mut socket: TcpStream) -> io::Result<()> {
    let (reader, mut writer) = socket.split();
    let mut reader = tokio::io::BufReader::new(reader);
    let mut msg: String = String::new();
    match reader.read_line(&mut msg).await {
        Ok(_bytes_size) => {
            // println!("[Server] received a message: {:?}", msg.trim());
            let mut msg = msg.trim();
            if msg.is_empty() {
                msg = "PONG";
            }
            match writer.write_all(msg.as_bytes()).await {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

// =================================================================
// =================================================================
#[cfg(test)]
mod tests {
    use crate::client::Client;

    /// A PING PONG test without message provided.
    /// It should return "PONG".
    #[tokio::test]
    async fn ping_pong_without_message() {
        let mut client = Client::connect(config::BINDING_ADDRESS).await;
        let response = client.ping(None).await.unwrap();
        assert_eq!(response.as_bytes(), b"PONG");
    }

    /// A PING PONG test with message provided.
    /// It should return the message.
    #[tokio::test]
    async fn ping_pong_with_message() {
        let mut client = Client::connect(config::BINDING_ADDRESS).await;
        let response = client.ping(Some("你好世界")).await.unwrap();
        assert_eq!(response.as_bytes(), "你好世界".as_bytes());
    }

}
