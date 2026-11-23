
use std::{path::PathBuf, str::FromStr};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Server(ServerCommand),
    Client(ClientCommand),
}

impl From<ServerCommand> for Command { fn from(command: ServerCommand) -> Self { Command::Server(command) } }
impl From<ClientCommand> for Command { fn from(command: ClientCommand) -> Self { Command::Client(command) } }

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerCommand {
    Disconnect { client: String },
    Play { path: PathBuf },
    Clear,
    Exit,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientCommand {
    Identify { name: String },
    SubscribeLyrics,
}

impl Command {
    #[inline(always)]
    pub fn to_line(&self) -> String {
        let mut s = self.to_string();
        s.push('\n');
        s
    }
}

impl ToString for Command {
    #[inline(always)]
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl FromStr for Command {
    type Err = serde_json::Error;
    
    #[inline(always)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}
