use std::net::SocketAddr;

use crate::raw_tokio::EconRaw;

#[derive(Default)]
pub struct Econ {
    raw: Option<EconRaw>,
    is_alive: bool,
}

impl Econ {
    pub fn new() -> Self {
        Self::default()
    }

    /// Connects to given address
    pub async fn connect(&mut self, address: impl Into<SocketAddr>) -> std::io::Result<()> {
        self.raw = Some(EconRaw::connect(address, 2048, 5).await?);

        self.is_alive = true;

        Ok(())
    }

    pub async fn reconnect(&mut self) -> std::io::Result<()> {
        assert!(
            self.raw.is_some() && !self.is_alive,
            "can't reconnect without being disconnected"
        );

        let raw = unsafe { self.raw.as_mut().unwrap_unchecked() };

        raw.reconnect().await
    }

    /// Disconnects from econ on connection
    pub async fn disconnect(&mut self) -> std::io::Result<()> {
        let raw = self.raw.as_mut().unwrap();

        raw.disconnect().await
    }

    /// Tries to authenticate, returns `false` if password is incorrect
    pub async fn try_auth(&mut self, password: impl Into<String>) -> std::io::Result<bool> {
        let raw = self.get_raw_mut();

        Ok(raw.auth(password.into().as_str()).await?)
    }

    /// Change auth message
    pub fn set_auth_message<T: ToString>(&mut self, auth_message: T) {
        let raw = self.get_raw_mut();

        raw.set_auth_message(auth_message.to_string());
    }

    /// Non-blocking *write* operation, sends line to socket
    pub async fn send_line(&mut self, line: impl Into<String>) -> std::io::Result<()> {
        let raw = self.get_raw_mut();

        assert!(raw.is_authed(), "can't send commands without being authed");

        raw.try_send(line.into().as_str()).await
    }

    /// Non-blocking *read* operation, reads to buffer and appends to inner line buffer
    pub async fn fetch(&mut self) -> std::io::Result<()> {
        let raw = self.get_raw_mut();

        assert!(raw.is_authed(), "can't fetch lines without being authed");

        raw.try_read().await?;

        Ok(())
    }

    /// Pops line from inner line buffer
    pub fn pop_line(&mut self) -> Option<String> {
        let raw = self.get_raw_mut();

        raw.pop_line()
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    fn get_raw_mut(&mut self) -> &mut EconRaw {
        assert!(
            self.raw.is_some() && self.is_alive,
            "can't do anything without being connected"
        );

        unsafe { self.raw.as_mut().unwrap_unchecked() }
    }
}
