use godot::prelude::*;
use crate::player::player::Player;
use crate::player::player_input::PLAYER_JUMP;

impl Player {
    pub (super) fn update_move(&mut self, delta: f64) {
        let mut velocity = self.base.get_velocity();

        if !self.base.is_on_floor() {
            velocity.y += (&self.gravity * delta) as f32;
        } else {
            self.jump_count = 0;
        }

        let player_speed = self.player_floor_speed;
        let player_velocity = self.player_floor_velocity;
        let player_stop_velocity = self.player_stop_floor_velocity;
        let input = Input::singleton();

        if input.is_action_just_pressed(PLAYER_JUMP.clone()) && self.jump_count < self.jump_count_limit {
            velocity.y = self.jump_velocity.clone() as real;
            self.jump_count += 1;
        }

        let direction = input.get_axis(StringName::from("ui_left"), StringName::from("ui_right"));
        if direction != 0.0 {
            velocity.x = velocity.x.lerp(&direction * player_speed, player_velocity * delta as f32) as real;
        } else {
            velocity.x = utilities::move_toward(velocity.x as f64, 0f64, player_stop_velocity * delta) as real
        }

        self.base.set_velocity(velocity);
        self.base.move_and_slide();
    }
}