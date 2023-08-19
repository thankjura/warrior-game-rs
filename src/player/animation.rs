use std::ops::DerefMut;
use godot::prelude::*;
use crate::player::player::Player;
use crate::player::player_state::PlayerState;

impl Player {
    pub (super) fn animation_state(&mut self, prev_state: &PlayerState) {
        if &self.state == prev_state {
            return;
        }
        if let Some(sprite) = &mut self.sprite {
            let sprite = sprite.deref_mut();
            match self.state {
                PlayerState::Idle => {
                    sprite.set_animation(StringName::from("idle"));
                }
                PlayerState::Run => {
                    sprite.set_animation(StringName::from("run"));
                }
                PlayerState::JumpUp => {
                    sprite.set_animation(StringName::from("jump"));
                }
                PlayerState::Fall => {
                    sprite.set_animation(StringName::from("fall"));
                }
                PlayerState::Attack => {
                    sprite.set_animation(StringName::from("attack"));
                }
            }
            sprite.play();
        }
    }

    pub (super) fn animation_finished(&mut self) {
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
                if x > 0.0 {
                    self.sprite.as_deref_mut().unwrap().set_flip_h(false);
                } else if x < 0.0 {
                    self.sprite.as_deref_mut().unwrap().set_flip_h(true);
                }
            }
            _ => {}
        }



        // match self.state {
        //     PlayerState::Idle => {
        //
        //     }
        //     PlayerState::Run => {}
        //     PlayerState::Air => {}
        //     PlayerState::Attack => {}
        // }

        // let velocity = self.base.get_velocity();
        // let sprite = self.sprite.as_deref_mut().unwrap();
        // let old_animation = sprite.get_animation();
        // let mut new_animation= old_animation.clone();
        //
        // if self.base.is_on_floor() {
        //     let input = Input::singleton();
        //     let direction = input.get_axis(StringName::from("ui_left"), StringName::from("ui_right"));
        //     if direction > 0.0 {
        //         new_animation = StringName::from("run");
        //         sprite.set_flip_h(false);
        //     } else if direction < 0.0 {
        //         new_animation = StringName::from("run");
        //         sprite.set_flip_h(true);
        //     } else {
        //         new_animation = StringName::from("idle");
        //     }
        // } else {
        //     if velocity.y > 0.0 {
        //         new_animation = StringName::from("fall");
        //     } else if velocity.y < 0.0 {
        //         new_animation = StringName::from("jump");
        //     }
        // }
        //
        // if !old_animation.eq(&new_animation) {
        //     sprite.set_animation(new_animation);
        //     sprite.play();
        // } else if new_animation.eq(&StringName::from("run")) && !sprite.is_playing() {
        //     sprite.play();
        // }
    }


}