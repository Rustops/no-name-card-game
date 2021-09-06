use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

use crate::clientinfo::ClientInfo;

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageLayer {
    Default,
    // In a typical client/server simulation, both the client and the server will
    // be exchanging messages at a constant rate. Laminar makes use of this by
    // packaging message acks with the next sent message. Therefore, in order for
    // reliability to work properly, we'll send a generic "ok" response.
    ResponseImOnline,

    /// From Server
    BroadcastToClients,
    ForwardChatMessage,
    PlayerEnterLobby,
    Order,

    /// From Client
    SendToServer,
    ConnectRequest,
    ChatMessage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransMessage {
    /// General Message
    Default(Message),
    /// Response to caller that I'm still online
    ResponseImOnline(Message),

    /// The default broadcast message type of the `Server`
    BroadcastToClients(Message),
    /// Forward chat messages received from client
    ForwardChatMessage(Message),
    /// Players enter the lobby
    PlayerEnterLobby(Message),

    /// Command message sent by the server to the client
    Order(Message),

    /// When a client requests a connection, the server processes it
    ConnectRequest(Message),
    /// The default send message type of the `Client`
    SendToServer(Message),
    /// The chat message submitted by the client
    ChatMessage(Message),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub from: ClientInfo,
    /// Message content
    /// NOTE: For more complex message content, consider use the MessageBody structure
    pub msg: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct MessageBody {
//     // layer: MessageLayer,
//     // msg: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub enum MessageLayer {
//     Chat,
// }

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageError {
    FromBytesError,
    SerdeJsonError,
}

impl From<serde_json::Error> for MessageError {
    fn from(_: serde_json::Error) -> Self {
        MessageError::SerdeJsonError
    }
}

pub type Error = MessageError;
pub type Result<T> = std::result::Result<T, MessageError>;

impl TransMessage {
    pub fn new(layer: MessageLayer, from: ClientInfo, msg: String) -> TransMessage {
        let msg = Message::new(from, msg);
        TransMessage::construct(layer, msg)
    }

    pub fn construct(layer: MessageLayer, m: Message) -> TransMessage {
        match layer {
            MessageLayer::Default => TransMessage::Default(m),
            MessageLayer::Order => TransMessage::Order(m),
            MessageLayer::ConnectRequest => TransMessage::ConnectRequest(m),
            MessageLayer::BroadcastToClients => TransMessage::BroadcastToClients(m),
            MessageLayer::SendToServer => TransMessage::SendToServer(m),
            MessageLayer::ChatMessage => TransMessage::ChatMessage(m),
            MessageLayer::ForwardChatMessage => TransMessage::ForwardChatMessage(m),
            MessageLayer::ResponseImOnline => TransMessage::ResponseImOnline(m),
            MessageLayer::PlayerEnterLobby => TransMessage::PlayerEnterLobby(m),
        }
    }

    pub fn serialize(&self) -> Result<String> {
        // Serialize `TransMessage` to data stream
        serde_json::to_string(&self).map_err::<Error, _>(Into::into)
    }

    pub fn update_layer(&self, layer: MessageLayer) -> Self {
        let m = match self {
            TransMessage::Default(m) => m,
            TransMessage::BroadcastToClients(m) => m,
            TransMessage::Order(m) => m,
            TransMessage::ConnectRequest(m) => m,
            TransMessage::SendToServer(m) => m,
            TransMessage::ChatMessage(m) => m,
            TransMessage::ForwardChatMessage(m) => m,
            TransMessage::ResponseImOnline(m) => m,
            TransMessage::PlayerEnterLobby(m) => m,
        };

        // Fixed msg, but change layer
        TransMessage::construct(layer, m.clone())
    }
}

impl Message {
    pub fn new(from: ClientInfo, msg: String) -> Message {
        Message { from, msg }
    }

    pub fn from_bytes(bytes: Bytes) -> Result<Message> {
        // Converting messages to human-readable form
        let p = bytes.to_vec();
        let s = String::from_utf8(p).unwrap();
        let msg: Vec<&str> = s.split('-').collect();
        // asset length == 2
        if msg.len() < 2 {
            Err(MessageError::FromBytesError)
        } else {
            // Due to the use of-separation, there will be a situation with a length of 3 or more
            // which can be simply dealt with for the time being, and this part can be optimized later.
            Ok(Message {
                from: msg[0].to_string().into(),
                msg: msg[1].to_string(),
            })
        }
    }
}

impl Display for TransMessage {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TransMessage::Default(m) => write!(f, "Default [{}]: {}", m.from, m.msg),
            TransMessage::BroadcastToClients(m) => write!(f, "S2C [{}]: {}", m.from, m.msg),
            TransMessage::Order(m) => write!(f, "Ord [{}]: {}", m.from, m.msg),
            TransMessage::ConnectRequest(m) => write!(f, "ConnReq [{}]: {}", m.from, m.msg),
            TransMessage::SendToServer(m) => write!(f, "C2S [{}]: {}", m.from, m.msg),
            TransMessage::ChatMessage(m) => write!(f, "Chat [{}]: {}", m.from, m.msg),
            TransMessage::ForwardChatMessage(m) => write!(f, "FChat [{}]: {}", m.from, m.msg),
            TransMessage::ResponseImOnline(m) => write!(f, "Online [{}]: {}", m.from, m.msg),
            TransMessage::PlayerEnterLobby(m) => write!(f, "InLobby [{}]: {}", m.from, m.msg),
        }
    }
}

#[cfg(test)]
mod message_tests {
    use super::*;
    #[test]
    fn test_construct_msg() {
        let mut m = "Client:9999".to_owned();

        let separator = "-";
        let content = "Broadcast content";
        m.push_str(separator);
        m.push_str(content);

        println!("[1] Before construct: {}", m);
        let msg = Message::from_bytes(Bytes::copy_from_slice(m.as_bytes())).unwrap();
        let trans_message = TransMessage::construct(MessageLayer::Default, msg);
        println!("[1] After  construct: {}", trans_message);

        let from = "Client:9999".to_owned();
        let content = "Broadcast content".to_owned();

        println!("[2] Before construct: {}", m);
        let msg = Message::new(from.into(), content);
        let trans_message = TransMessage::construct(MessageLayer::Default, msg);
        println!("[2] After  construct: {}", trans_message);
    }
}
