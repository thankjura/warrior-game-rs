use godot::prelude::*;
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
        if &self.state == &state {
            return;
        }

        let prev_state = std::mem::replace(&mut self.state, state);
        self.animation_state(&prev_state);

        let attack_area = self.attack_area.as_deref_mut().unwrap();
        attack_area.set_deferred(StringName::from("monitorable"), Variant::from(false));
        attack_area.set_deferred(StringName::from("monitoring"), Variant::from(false));
    }
}