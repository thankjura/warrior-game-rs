use godot::prelude::*;
use crate::player::player::Player;
use crate::player::player_input::{PLAYER_JUMP, PLAYER_LEFT, PLAYER_RIGHT};
use crate::player::player_state::PlayerState;

impl Player {
    pub(super) fn state_air_process(&mut self, delta: f64) {
        let mut velocity = self.base.get_velocity();

        if self.base.is_on_floor() {
            self.set_state(PlayerState::Idle);
            return;
        }

        velocity.y += (&self.gravity * delta) as f32;

        let input = Input::singleton();

        if input.is_action_just_pressed(PLAYER_JUMP.clone()) && self.jump_count < self.jump_count_limit  {
            velocity.y = self.jump_velocity.clone() as real;
            self.jump_count += 1;
        }

        if velocity.y >= 0.0 {
            self.set_state(PlayerState::Fall);
        } else {
            self.set_state(PlayerState::JumpUp);
        }

        let direction = input.get_axis(PLAYER_LEFT.clone(), PLAYER_RIGHT.clone());
        if direction != 0.0 {
            velocity.x = velocity.x.lerp(&direction * self.player_air_speed, self.player_air_velocity * delta as f32) as real;
        } else {
            velocity.x = utilities::move_toward(velocity.x as f64, 0f64, self.player_stop_air_velocity * delta) as real
        }
        self.base.set_velocity(velocity);
        self.base.move_and_slide();

        self.check_attack(delta);
    }
}