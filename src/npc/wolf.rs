use godot::engine::{AnimationPlayer, Area2D, CharacterBody2D, CharacterBody2DVirtual, Label, ProjectSettings, Sprite2D};
use godot::prelude::*;
use crate::npc::wolf_state::WolfState;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Wolf {
    pub (super) jump_velocity: f64,
    pub (super) gravity: f64,
    #[export]
    pub run_speed: f32,
    #[export]
    pub health: f32,

    #[base]
    pub (super) base: Base<CharacterBody2D>,

    pub (super) animation: Option<Gd<AnimationPlayer>>,
    pub (super) sprite: Option<Gd<Sprite2D>>,
    pub (super) head_area: Option<Gd<Area2D>>,
    pub (super) body_area: Option<Gd<Area2D>>,
    pub (super) label: Option<Gd<Label>>,

    pub (super) state: WolfState,
}

#[godot_api]
impl Wolf {
    #[func]
    pub fn on_animation_finished(&mut self, anim_name: StringName) {
        self.animation_finished(anim_name);
    }

    #[func]
    pub fn on_head_damage(&mut self, area: Gd<Area2D>) {
        self.damage(40.0);
    }

    #[func]
    pub fn on_body_damage(&mut self, area: Gd<Area2D>) {
        self.damage(20.0);
    }

    fn damage(&mut self, damage: f32) {
        if &self.state == &WolfState::Attack {
            return;
        }
        self.health -= damage;
        if self.health <= 0.0 {
            self.set_state(WolfState::Death);
            self.label.as_deref_mut().unwrap().queue_free();
        } else {
            self.set_state(WolfState::Damage);
            self.label.as_deref_mut().unwrap().set_text(GodotString::from(format!("{}", &self.health)));
        }
    }
}

#[godot_api]
impl CharacterBody2DVirtual for Wolf {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            jump_velocity: -300.0,
            gravity: ProjectSettings::singleton().get_setting(GodotString::from("physics/2d/default_gravity")).to(),
            run_speed: 180.0,
            health: 100.0,
            base,
            animation: None,
            sprite: None,
            head_area: None,
            body_area: None,
            label: None,
            state: WolfState::Idle,
        }
    }

    fn process(&mut self, delta: f64) {
        match self.state {
            WolfState::Idle => {
                self.state_idle_process(delta);
            }
            WolfState::Attack => {
                //self.set_state(WolfState::Attack);
            }
            WolfState::Death => {
                //self.set_state(WolfState::Death);
            }
            WolfState::Damage => {
                self.state_damage_process(delta);
            }
        }

        self.animation_process(delta);
    }

    fn ready(&mut self) {
        self.animation = Some(self.base.get_node_as("anim"));
        self.sprite = Some(self.base.get_node_as("sprite"));
        self.head_area = Some(self.base.get_node_as("head_area"));
        self.body_area = Some(self.base.get_node_as("body_area"));
        self.label = Some(self.base.get_node_as("label"));
        self.label.as_deref_mut().unwrap().set_text(GodotString::from(format!("{}", self.health)));

        let callable = self.base.callable("on_animation_finished");
        self.animation.as_deref_mut().unwrap().connect(StringName::from("animation_finished"), callable);

        let head_damage_callable = self.base.callable("on_head_damage");
        let body_damage_callable = self.base.callable("on_body_damage");
        self.head_area.as_deref_mut().unwrap().connect(StringName::from("area_entered"), head_damage_callable);
        self.body_area.as_deref_mut().unwrap().connect(StringName::from("area_entered"), body_damage_callable);
    }
}