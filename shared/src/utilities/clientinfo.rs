use std::fmt::Display;

use serde::{Deserialize, Serialize};

const CLIENT_NAME: &str = "test";
const UDP_PORT: u16 = 2000;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClientInfo {
    pub name: String,
    pub port: u16,
}

impl ClientInfo {
    pub fn new(name: String, port: u16) -> Self {
        Self { name, port }
    }
}

impl Default for ClientInfo {
    fn default() -> Self {
        Self {
            name: CLIENT_NAME.to_string(),
            port: UDP_PORT,
        }
    }
}

impl Display for ClientInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.name, self.port)
    }
}

impl From<String> for ClientInfo {
    fn from(s: String) -> Self {
        let info: Vec<&str> = s.split(':').collect();
        let name = info[0].to_string();
        let port = info[1].parse::<u16>().unwrap_or_default();
        Self { name, port }
    }
}
