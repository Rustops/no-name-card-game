use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

use crate::clientinfo::ClientInfo;

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageLayer {
    // information in info box
    System,
    // manage connections
    Connection,
    // process chat message
    Chat,
    // process lobby state
    Lobby,
    // process information in game
    Game,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransMessage {
    Connection(Message),
    System(Message),
    Lobby(Message),
    Chat(Message),
    Game(Message),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub from: ClientInfo,
    /// Message content
    /// NOTE: For more complex message content, consider use the MessageBody structure
    pub msg_type: MessageType,
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageType {
    ExitLobby,
    EnterLobby,
    Prepare,
    CancelPrepare,
    Chat,
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
    pub fn new(
        layer: MessageLayer,
        from: ClientInfo,
        msg_type: MessageType,
        msg: String,
    ) -> TransMessage {
        let msg = Message::new(from, msg_type, msg);
        TransMessage::construct(layer, msg)
    }

    pub fn construct(layer: MessageLayer, m: Message) -> TransMessage {
        match layer {
            MessageLayer::System => TransMessage::System(m),
            MessageLayer::Connection => TransMessage::Connection(m),
            MessageLayer::Chat => TransMessage::Chat(m),
            MessageLayer::Lobby => TransMessage::Lobby(m),
            MessageLayer::Game => TransMessage::Game(m),
        }
    }

    pub fn serialize(&self) -> Result<String> {
        // Serialize `TransMessage` to data stream
        serde_json::to_string(&self).map_err::<Error, _>(Into::into)
    }

    pub fn update_layer(&self, layer: MessageLayer) -> Self {
        let m = match self {
            TransMessage::System(m) => m,
            TransMessage::Connection(m) => m,
            TransMessage::Chat(m) => m,
            TransMessage::Lobby(m) => m,
            TransMessage::Game(m) => m,
        };

        // Fixed msg, but change layer
        TransMessage::construct(layer, m.clone())
    }
}

impl Message {
    pub fn new(from: ClientInfo, msg_type: MessageType, msg: String) -> Message {
        Message {
            from,
            msg_type,
            msg,
        }
    }

    // pub fn from_bytes(bytes: Bytes) -> Result<Message> {
    //     // Converting messages to human-readable form
    //     let p = bytes.to_vec();
    //     let s = String::from_utf8(p).unwrap();
    //     let msg: Vec<&str> = s.split('-').collect();
    //     // asset length == 2
    //     if msg.len() < 2 {
    //         Err(MessageError::FromBytesError)
    //     } else {
    //         // Due to the use of-separation, there will be a situation with a length of 3 or more
    //         // which can be simply dealt with for the time being, and this part can be optimized later.
    //         Ok(Message {
    //             from: msg[0].to_string().into(),
    //             msg: msg[1].to_string(),
    //         })
    //     }
    // }
}

impl Display for TransMessage {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TransMessage::System(m) => write!(f, "Default [{}]: {}", m.from, m.msg),
            TransMessage::Connection(m) => write!(f, "S2C [{}]: {}", m.from, m.msg),
            TransMessage::Chat(m) => write!(f, "Ord [{}]: {}", m.from, m.msg),
            TransMessage::Lobby(m) => write!(f, "ConnReq [{}]: {}", m.from, m.msg),
            TransMessage::Game(m) => write!(f, "C2S [{}]: {}", m.from, m.msg),
        }
    }
}

// #[cfg(test)]
// mod message_tests {
//     use super::*;
//     #[test]
//     fn test_construct_msg() {
//         let mut m = "Client:9999".to_owned();

//         let separator = "-";
//         let content = "Broadcast content";
//         let msg_type=  MessageType::Chat;
//         m.push_str(separator);

//         m.push_str(content);

//         println!("[1] Before construct: {}", m);
//         let msg = Message::from_bytes(Bytes::copy_from_slice(m.as_bytes())).unwrap();
//         let trans_message = TransMessage::construct(MessageLayer::System, msg);
//         println!("[1] After  construct: {}", trans_message);

//         let from = "Client:9999".to_owned();
//         let content = "Broadcast content".to_owned();

//         println!("[2] Before construct: {}", m);
//         let msg = Message::new(from.into(), content);
//         let trans_message = TransMessage::construct(MessageLayer::System, msg);
//         println!("[2] After  construct: {}", trans_message);
//     }
// }
