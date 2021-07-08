use gdnative::prelude::*;
#[derive(FromVariant, ToVariant, PartialEq, Eq, Debug, Copy, Clone)]
pub enum State {
    IDLE,
    MOVE,
    JUMP,
    FALL,
    DEAD,
    HURT,
    ATTACK,
}
