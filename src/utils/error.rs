
#[derive(Clone, PartialEq)]
pub enum WSError {
    InvalidData(String)
}

impl WSError {
    pub fn description(&self) -> String {
        match *self {
            WSError::InvalidData(ref e) => format!("invalid data was provided: {}", e)
        }
    }
}

impl std::fmt::Display for WSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.description().as_str())
    }
}

impl std::fmt::Debug for WSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("websocket_next::Error")
            .field(&self.description())
            .finish()
    }
}

impl std::error::Error for WSError {}
