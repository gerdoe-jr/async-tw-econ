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

    /// Disconnects from econ on connection
    pub fn disconnect(&mut self) -> std::io::Result<()> {
        assert!(
            self.raw.is_some(),
            "you can't disconnect without being connected"
        );

        let raw = self.raw.as_mut().unwrap();

        raw.disconnect()
    }

    /// Tries to authenticate, returns `false` if password is incorrect
    pub async fn try_auth(&mut self, password: impl Into<String>) -> std::io::Result<bool> {
        let raw = self.get_raw_mut();

        Ok(raw.auth(password.into().as_str()).await?)
    }

    /// Non-blocking *write* operation, sends line to socket
    pub async fn send_line(&mut self, line: impl Into<String>) -> std::io::Result<()> {
        let raw = self.get_raw_mut();

        assert!(
            raw.is_authed(),
            "you can't send commands without being authed"
        );

        raw.try_send(line.into().as_str()).await
    }

    /// Non-blocking *read* operation, reads to buffer and appends to inner line buffer
    pub async fn fetch(&mut self) -> std::io::Result<()> {
        let raw = self.get_raw_mut();

        assert!(
            raw.is_authed(),
            "you can't fetch lines without being authed"
        );

        raw.try_read().await?;

        Ok(())
    } 

    /// Pops line from inner line buffer
    pub fn pop_line(&mut self) -> Option<String> {
        let raw = self.get_raw_mut();

        raw.pop_line()
    }

    fn get_raw_mut(&mut self) -> &mut EconRaw {
        assert!(
            self.raw.is_some(),
            "you can't fetch lines without being connected"
        );

        unsafe { self.raw.as_mut().unwrap_unchecked() }
    }
}
