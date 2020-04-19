use serde::{Serialize, Deserialize};
use serde_json::Value;
use serde_repr::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Payload {
    pub op: OpCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e: Option<EventName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<String>
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    Dispatch = 0,
    Hello = 1,
    Identify = 2,
    Reconnect = 3,
    GuildSubscribe = 4,
    GuildUnsubscribe = 5,
    SubscriptionAck = 6,
    GuildRequest = 7
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EventName {
    #[serde(rename = "READY")]
    Ready,
    #[serde(rename = "GUILD_ACCESS_RECEIVED")]
    GuildAccessReceived,
    #[serde(rename = "GUILD_ACCESS_REVOKED")]
    GuildAccessRevoked,
    #[serde(rename = "GUILD_DELETE")]
    GuildDelete,
    #[serde(rename = "GUILD_CHANNEL_CREATE")]
    GuildChannelCreate,
    #[serde(rename = "GUILD_CHANNEL_UPDATE")]
    GuildChannelUpdate,
    #[serde(rename = "GUILD_CHANNEL_DELETE")]
    GuildChannelDelete,
    #[serde(rename = "GUILD_ROLE_CREATE")]
    GuildRoleCreate,
    #[serde(rename = "GUILD_ROLE_UPDATE")]
    GuildRoleUpdate,
    #[serde(rename = "GUILD_ROLE_DELETE")]
    GuildRoleDelete,
    #[serde(rename = "GUILD_DATA")]
    GuildData,
    #[serde(rename = "GUILD_UPDATE")]
    GuildUpdate,
    #[serde(rename = "PARTIAL_GUILD_UPDATE")]
    PartialGuildUpdate
}

impl Payload {
    pub fn from_event(payload: Value, event: EventName) -> Self {
        Self {
            op: OpCode::Dispatch,
            d: Some(payload),
            e: Some(event),
            s: None
        }
    }

    pub fn from_op(payload: Value, op: OpCode) -> Self {
        Self {
            op,
            d: Some(payload),
            e: None,
            s: None
        }
    }

    pub fn from_bare_op(op: OpCode) -> Self {
        Self {
            op,
            d: None,
            e: None,
            s: None
        }
    }
}
