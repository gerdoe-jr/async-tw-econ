use std::net::SocketAddr;

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

pub struct EconRaw {
    socket: TcpStream,
    buffer: Vec<u8>,
    lines: Vec<String>,
    unfinished_line: String,
    authed: bool,
}

impl EconRaw {
    pub async fn connect(
        address: impl Into<SocketAddr>,
        buffer_size: usize,
        _timeout_secs: u64,
    ) -> std::io::Result<Self> {
        let buffer = vec![0u8; buffer_size];
        let address = address.into();

        let connection = TcpStream::connect(&address).await?;

        Ok(Self {
            socket: connection,
            buffer,
            lines: Vec::new(),
            unfinished_line: String::new(),
            authed: false,
        })
    }

    pub fn disconnect(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    pub async fn auth(&mut self, password: &str) -> std::io::Result<bool> {
        self.read().await?;
        self.lines.clear();

        self.send(password).await?;

        self.read().await?;

        while let Some(line) = self.pop_line() {
            if line.starts_with("Authentication successful") {
                self.authed = true;
            }
        }

        Ok(self.authed)
    }

    pub async fn read(&mut self) -> std::io::Result<usize> {
        let mut lines_amount = 0;
        let written = self.socket.read(&mut self.buffer).await?;

        if written != 0 {
            let mut lines: Vec<String> = String::from_utf8_lossy(&self.buffer[..written])
                .replace('\0', "")
                .split("\n")
                .map(String::from)
                .collect();

            if lines.last().unwrap() == "" {
                let _ = lines.pop();

                if !self.unfinished_line.is_empty() {
                    let take = self.unfinished_line.to_owned();
                    lines[0] = take + &lines[0];

                    self.unfinished_line.clear();
                }
            } else {
                self.unfinished_line = lines.pop().unwrap();
            }

            lines_amount = lines.len();

            self.lines.extend(lines);
        }

        Ok(lines_amount)
    }

    pub async fn send(&mut self, line: &str) -> std::io::Result<()> {
        self.socket.write_all(line.as_bytes()).await?;
        self.socket.write_all("\n".as_bytes()).await?;

        self.socket.flush().await?;

        Ok(())
    }

    pub fn pop_line(&mut self) -> Option<String> {
        self.lines.pop()
    }

    pub fn is_authed(&self) -> bool {
        self.authed
    }
}