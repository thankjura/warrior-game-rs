use godot::prelude::*;
use crate::npc::wolf::Wolf;

#[derive(PartialEq, Debug)]
pub enum WolfState {
    Idle,
    Attack,
    Death,
    Damage,
}

impl Wolf {
    pub(super) fn set_state(&mut self, state: WolfState) {
        if &self.state == &state {
            return;
        }

        let prev_state = std::mem::replace(&mut self.state, state);
        self.animation_state(&prev_state);
        if &WolfState::Attack == &self.state {
            if self.sprite.as_deref().unwrap().is_flipped_h() {
                self.head_area.as_deref_mut().unwrap().set_scale(Vector2::new(-1.0, 1.0));
                self.body_area.as_deref_mut().unwrap().set_scale(Vector2::new(-1.0, 1.0));
            } else {
                self.head_area.as_deref_mut().unwrap().set_scale(Vector2::new(1.0, 1.0));
                self.body_area.as_deref_mut().unwrap().set_scale(Vector2::new(1.0, 1.0));
            }
        }
        else if &WolfState::Death== &self.state {
            // self.head_area.as_deref_mut().unwrap().set_monitoring(false);
            // self.body_area.as_deref_mut().unwrap().set_monitorable(false);
            self.base.set_z_index(-1);
            self.body_area.as_deref_mut().unwrap().set_deferred(StringName::from("monitorable"), Variant::from(false));
            self.body_area.as_deref_mut().unwrap().set_deferred(StringName::from("monitoring"), Variant::from(false));
        }
    }
}