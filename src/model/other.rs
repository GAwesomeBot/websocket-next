use std::convert::TryFrom;
use actix_web_actors::ws;
use crate::utils::error::WSError;

pub enum CloseCodes {
    InvalidOpCode,
    InvalidPacket,
    NotAuthenticated,
    MissingSessionID,
    InvalidSessionID,
    InvalidUserToken,
    HeartbeatTimeout,
    IdentifyTimeout
}

impl CloseCodes {
    pub fn as_u16(&self) -> u16 {
        match self {
            Self::InvalidOpCode => 4001,
            Self::InvalidPacket => 4002,
            Self::NotAuthenticated => 4003,
            Self::MissingSessionID => 4004,
            Self::InvalidSessionID => 4005,
            Self::InvalidUserToken => 4006,
            Self::HeartbeatTimeout => 4008,
            Self::IdentifyTimeout => 4009
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::InvalidOpCode => "you sent a non-existent op code or invalid data for an op code. Don't do that!",
            Self::InvalidPacket => "you sent us an invalid payload. Don't do that!",
            Self::NotAuthenticated => "you tried sending a payload before identifying.",
            Self::MissingSessionID => "you sent us a packet without a session ID. Don't do that!",
            Self::InvalidSessionID => "the session ID you used is not valid. Reconnect and try again",
            Self::InvalidUserToken => "the user token provided is not valid.",
            Self::HeartbeatTimeout => "you didn't answer the heartbeat in time, please reconnect",
            Self::IdentifyTimeout => "you didn't identify in time. Try again"
        }
    }
}

impl From<CloseCodes> for u16 {
    fn from(cc: CloseCodes) -> u16 {
        cc.as_u16()
    }
}

impl TryFrom<u16> for CloseCodes {
    type Error = WSError;

    fn try_from(int: u16) -> Result<CloseCodes, Self::Error> {
        match int {
            4001 => Ok(Self::InvalidOpCode),
            4002 => Ok(Self::InvalidPacket),
            4003 => Ok(Self::NotAuthenticated),
            4004 => Ok(Self::MissingSessionID),
            4005 => Ok(Self::InvalidSessionID),
            4006 => Ok(Self::InvalidUserToken),
            4008 => Ok(Self::HeartbeatTimeout),
            4009 => Ok(Self::IdentifyTimeout),
            _ => Err(WSError::InvalidData("Invalid Close Code".into()))
        }
    }
}

impl From<CloseCodes> for ws::CloseReason {
    fn from(cc: CloseCodes) -> Self {
        Self {
            code: ws::CloseCode::from(cc.as_u16()),
            description: Some(cc.description().to_owned())
        }
    }
}
