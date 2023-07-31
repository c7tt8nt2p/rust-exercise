use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut client = Client::connect(config::BINDING_ADDRESS).await;

    let stdin = tokio::io::stdin();
    let mut lines = BufReader::new(stdin).lines();

    while let Ok(line) = lines.next_line().await {
        client.ping(line.as_deref()).await?;
    }

    Ok(())
}

pub struct Client {
    connection: BufReader<TcpStream>,
}

impl Client {
    pub async fn connect(address: &str) -> Self {
        let stream = TcpStream::connect(address).await.unwrap();
        let stream = BufReader::new(stream);
        Self { connection: stream }
    }

    pub async fn ping(&mut self, text: Option<&str>) -> io::Result<String> {
        let text: String = text.map_or("\r\n".to_string(), |e| e.to_string() + "\r\n");
        self.connection.write_all(text.as_bytes()).await?;

        let mut response = String::new();
        self.connection.read_to_string(&mut response).await?;
        println!("server says: {:?}", response);
        Ok(response)
    }
}
