use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub channels: Vec<Channel>,
    pub roles: Vec<Role>,
    pub settings: String,
    pub owner: String,
    pub member_count: i32,
    pub shard: i32,
    pub authed_user: Member,
    pub bot_permissions: u64
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Channel {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub channel_type: u8,
    pub parent_id: String,
    pub position: i32,
    pub calculated_permissions: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub position: i32,
    pub permission: u64,
    pub color: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Member {
    pub nickname: Option<String>,
    pub roles: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    pub id: String,
    pub username: String,
    pub icon: String,
    pub discriminator: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PartialGuild {
    pub id: String,
    pub name: String,
    pub icon: String,
}
