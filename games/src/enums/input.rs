use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Direction {
    pub(crate) up: bool,
    pub(crate) down: bool,
    pub(crate) left: bool,
    pub(crate) right: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Inputs {
    // 移动
    Direction(Direction),
    // 跳跃
    Jump,
    // 触发
    Action,
    // 没有任何输入
    None,
}
impl UserAction for Inputs {}
