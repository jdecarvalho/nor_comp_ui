use serde::{Deserialize, Serialize};

pub enum InboundMessageEvent {
    InboundMessage(String),
}

#[derive(Deserialize)]
pub enum InboundMessageType {
    RegisterModified {
        register_name: String,
    },
    OutputValue,
}

#[derive(Deserialize)]
pub struct InboundMessage {
    pub message_type: InboundMessageType,
    pub data: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum OutboundMessageType {
    ComputerReset,
}

#[derive(Serialize, Deserialize)]
pub struct OutboundMessage {
    pub message_type: OutboundMessageType,
    pub data: Option<String>,
}
