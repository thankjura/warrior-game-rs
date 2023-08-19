use crate::player::player::Player;

#[derive(PartialEq, Debug)]
pub enum PlayerState {
    Idle,
    Run,
    JumpUp,
    Fall,
    Attack,
}

impl Player {
    pub(super) fn set_state(&mut self, state: PlayerState) {
        let prev_state = std::mem::replace(&mut self.state, state);
        self.animation_state(&prev_state);
    }
}