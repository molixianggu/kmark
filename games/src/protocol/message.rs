use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

use super::protocol::MyProtocol;

#[derive(Message, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Hello {
    pub message: u32,
}

#[message_protocol(protocol = "MyProtocol")]
pub enum Messages {
    Hello(Hello),
}
