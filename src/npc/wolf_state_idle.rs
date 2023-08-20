use godot::prelude::*;
use crate::npc::wolf::Wolf;

impl Wolf {
    pub(super) fn state_idle_process(&mut self, delta: f64) {
        let mut velocity = self.base.get_velocity();

        // if !self.base.is_on_floor() {
        //     self.set_state(WolfState::Fall);
        //     return;
        // }

        // if velocity.x != 0.0 {
        //     self.set_state(PlayerState::Run);
        //     return;
        // }
        velocity.y += (&self.gravity * delta) as f32;
        velocity.x = utilities::move_toward(velocity.x as f64, 0f64, self.run_speed as f64 * delta) as real;
        self.base.set_velocity(velocity);
        self.base.move_and_slide();
    }
}