use crate::enums::State;
use gdnative::prelude::*;

#[derive(Copy, Clone, ToVariant, FromVariant)]
pub struct RuntimeData {
    pub current_state: State,
    pub next_state: State,
    pub prev_state: State,
}
impl RuntimeData {
    pub fn new() -> Self {
        RuntimeData {
            current_state: State::IDLE,
            next_state: State::IDLE,
            prev_state: State::IDLE,
        }
    }

    pub fn change_current_state(&mut self, state: State) {
        self.current_state = state;
    }
    pub fn change_next_state(&mut self, state: State) {
        self.next_state = state;
    }
    pub fn change_prev_state(&mut self, state: State) {
        self.prev_state = state;
    }
}
