use godot::prelude::*;
use crate::player::player::Player;
use crate::player::player_input::{PLAYER_JUMP, PLAYER_LEFT, PLAYER_RIGHT};
use crate::player::player_state::PlayerState;

impl Player {
    pub(super) fn state_run_process(&mut self, delta: f64) {
        let mut velocity = self.base.get_velocity();

        if !self.base.is_on_floor() {
            self.set_state(PlayerState::Fall);
            return;
        }

        if velocity.x == 0.0 {
            self.set_state(PlayerState::Idle);
            return;
        }

        self.jump_count = 0;

        let input = Input::singleton();

        if input.is_action_just_pressed(PLAYER_JUMP.clone()) {
            velocity.y = self.jump_velocity.clone() as real;
            self.jump_count += 1;
        }
        let direction = input.get_axis(PLAYER_LEFT.clone(), PLAYER_RIGHT.clone());
        if direction != 0.0 {
            velocity.x = velocity.x.lerp(&direction * self.player_floor_speed, self.player_floor_velocity * delta as f32) as real;
        } else {
            velocity.x = utilities::move_toward(velocity.x as f64, 0f64, self.player_stop_floor_velocity * delta) as real;
        }
        self.base.set_velocity(velocity);
        self.base.move_and_slide();

        self.check_attack(delta);
    }
}