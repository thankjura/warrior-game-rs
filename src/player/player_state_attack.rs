use godot::prelude::*;
use crate::player::player::Player;
use crate::player::player_input::PLAYER_ATTACK;
use crate::player::player_state::PlayerState;

impl Player {
    pub (super) fn check_attack(&mut self, delta: f64) {
        let input = Input::singleton();

        if input.is_action_just_pressed(PLAYER_ATTACK.clone()) {
            self.set_state(PlayerState::Attack);
        }
    }

    pub(super) fn state_attack_process(&mut self, delta: f64) {
        let mut velocity = self.base.get_velocity();

        if self.base.is_on_floor() {
            velocity.x = utilities::move_toward(velocity.x as f64, 0f64, self.player_stop_floor_velocity * delta) as real
        } else {
            velocity.y += (&self.gravity * delta) as f32;
            velocity.x = utilities::move_toward(velocity.x as f64, 0f64, self.player_stop_air_velocity * delta) as real
        }
        self.base.set_velocity(velocity);
        self.base.move_and_slide();
    }
}