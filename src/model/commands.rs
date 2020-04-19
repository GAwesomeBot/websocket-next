use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Hello {
    pub heartbeat: u64,
    pub session_id: uuid::Uuid
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Identify {
    pub token: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SubscribeToGuild {
    pub subscribe_to: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestGuildData {
    pub guild_id: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SubscriptionAck {
    pub subscribed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>
}
