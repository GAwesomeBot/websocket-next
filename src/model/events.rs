use serde::{Serialize, Deserialize};
use super::discord;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Ready {
    pub user: discord::User,
    pub guilds: Vec<discord::PartialGuild>
}
