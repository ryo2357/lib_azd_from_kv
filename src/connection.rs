use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use super::config::AzdFromKvConfig;

pub struct HotLinkConnection {
    connection: TcpStream,
}

impl HotLinkConnection {
    pub async fn connect(config: &AzdFromKvConfig) -> anyhow::Result<Self> {
        let connection = TcpStream::connect(&config.address).await?;

        Ok(Self { connection })
    }

    pub async fn send_command(&mut self, command: String) -> anyhow::Result<String> {
        self.connection.write_all(command.as_bytes()).await?;

        let mut buf = Vec::with_capacity(4096);
        self.connection.read_buf(&mut buf).await?;

        let response = String::from_utf8(buf).unwrap();
        Ok(response)
    }
}
