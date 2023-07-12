use tokio::io;
use tokio::io::BufReader;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

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
