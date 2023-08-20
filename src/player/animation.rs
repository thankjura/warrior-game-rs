use std::ops::DerefMut;
use godot::prelude::*;
use crate::player::player::Player;
use crate::player::player_state::PlayerState;

impl Player {
    pub (super) fn animation_state(&mut self, _prev_state: &PlayerState) {
        if let Some(animation) = &mut self.animation {
            let animation = animation.deref_mut();
            match self.state {
                PlayerState::Idle => {
                    animation.set_current_animation(GodotString::from("idle"));
                }
                PlayerState::Run => {
                    animation.set_current_animation(GodotString::from("run"));
                }
                PlayerState::JumpUp => {
                    animation.set_current_animation(GodotString::from("jump"));
                }
                PlayerState::Fall => {
                    animation.set_current_animation(GodotString::from("fall"));
                }
                PlayerState::Attack => {
                    animation.set_current_animation(GodotString::from("attack"));
                }
            }
            animation.play();
        }
    }

    pub (super) fn animation_finished(&mut self, _anim_name: StringName) {
        match self.state {
            PlayerState::Attack => {
                self.set_state(PlayerState::Idle);
            }
            _ => {}
        }
    }

    pub (super) fn animation_process(&mut self, _delta: f64) {
        match self.state {
            PlayerState::Run|PlayerState::JumpUp|PlayerState::Fall => {
                let x= self.base.get_velocity().x;
                let mut sprite = self.sprite.as_deref_mut().unwrap();
                if x > 0.0 {
                    sprite.set_flip_h(false);
                } else if x < 0.0 {
                    sprite.set_flip_h(true);
                }
            }
            _ => {}
        }
    }
}