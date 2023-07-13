use futures_util::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let mut ws_stream = ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
        .connect()
        .await?;

    let stdin = tokio::io::stdin();
    let mut lines = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            // Receiving data from server
            Some(result) = ws_stream.next() => {
                // Received message from the server
                let message = result.expect("Failed to receive message");
                let message = message.as_text().unwrap();
                println!("Received message from server: {:?}", message);
            },
            // Sending data to server
            Ok(Some(line)) = lines.next_line() => {
                let message = Message::text(line.clone());
                ws_stream.send(message).await.expect("Failed to send message");
                println!("Sent message to server: {}", line);
            }
        }
    }
}
