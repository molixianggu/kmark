use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct InputDirection {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl InputDirection {
    pub fn is_none(&self) -> bool {
        !self.up && !self.down && !self.left && !self.right
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Inputs {
    // 移动
    Direction(InputDirection),
    // 跳跃
    Jump,
    // 触发
    Action,
    // 没有任何输入
    None,
}
impl UserAction for Inputs {}
