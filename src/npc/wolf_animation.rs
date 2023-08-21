use std::ops::DerefMut;
use godot::prelude::*;
use crate::npc::wolf::Wolf;
use crate::npc::wolf_state::WolfState;

impl Wolf {
    pub (super) fn animation_state(&mut self, prev_state: &WolfState) {
        if let Some(animation) = &mut self.animation {
            let animation = animation.deref_mut();
            match self.state {
                WolfState::Idle => {
                    animation.set_current_animation(GodotString::from("idle"));
                }
                WolfState::Attack => {
                    animation.set_current_animation(GodotString::from("attack"));
                }
                WolfState::Damage => {
                    animation.set_current_animation(GodotString::from("damage"));
                }
                WolfState::Death => {
                    animation.set_current_animation(GodotString::from("death"));
                }
            }
            animation.play();
        }
    }

    pub (super) fn animation_finished(&mut self, _anim_name: StringName) {
        match self.state {
            WolfState::Damage => {
                self.set_state(WolfState::Idle);
            }
            _ => {}
        }
    }

    pub (super) fn animation_process(&mut self, _delta: f64) {
        // match self.state {
        //     PlayerState::Run|PlayerState::JumpUp|PlayerState::Fall => {
        //         let x= self.base.get_velocity().x;
        //         let mut sprite = self.sprite.as_deref_mut().unwrap();
        //         if x > 0.0 {
        //             sprite.set_flip_h(false);
        //         } else if x < 0.0 {
        //             sprite.set_flip_h(true);
        //         }
        //     }
        //     _ => {}
        // }
    }
}