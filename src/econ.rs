use std::net::SocketAddr;

#[cfg(feature = "async-std")]
use crate::raw_async_std::EconRaw;

#[cfg(feature = "tokio")]
use crate::raw_tokio::EconRaw;

pub struct Econ {
    raw: Option<EconRaw>,
}

impl Econ {
    pub fn new() -> Self {
        Self { raw: None }
    }

    /// Connects to given address
    pub async fn connect(&mut self, address: impl Into<SocketAddr>) -> std::io::Result<()> {
        self.raw = Some(EconRaw::connect(address, 2048, 5).await?);

        Ok(())
    }

    pub fn disconnect(&mut self) -> std::io::Result<()> {
        assert!(
            self.raw.is_some(),
            "you can't disconnect without being connected"
        );

        let raw = self.raw.as_mut().unwrap();

        raw.disconnect()
    }

    /// Tries to authenticate, returns `false` if password is incorrect`
    pub async fn try_auth(&mut self, password: impl Into<String>) -> std::io::Result<bool> {
        assert!(
            self.raw.is_some(),
            "you can't authenticate without being connected"
        );

        let raw = self.raw.as_mut().unwrap();

        Ok(raw.auth(password.into().as_str()).await?)
    }

    /// Blocking *write* operation, sends line to socket
    pub async fn send_line(&mut self, line: impl Into<String>) -> std::io::Result<()> {
        assert!(
            self.raw.is_some(),
            "you can't send commands without being connected"
        );

        let raw = self.raw.as_mut().unwrap();

        assert!(
            raw.is_authed(),
            "you can't send commands without being authed"
        );

        raw.send(line.into().as_str()).await
    }

    /// Blocking *read* operation, reads to buffer and appends to inner line buffer
    /// if fetch set to `true`, otherwise returns popped line from line buffer
    /// with no another operation
    pub async fn recv_line(&mut self, fetch: bool) -> std::io::Result<Option<String>> {
        assert!(
            self.raw.is_some(),
            "you can't fetch lines without being connected"
        );

        let raw = self.raw.as_mut().unwrap();

        if fetch == true {
            assert!(
                raw.is_authed(),
                "you can't fetch lines without being authed"
            );

            raw.read().await?;
        }

        Ok(raw.pop_line())
    }
}
