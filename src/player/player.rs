use godot::prelude::*;
use godot::engine::{AnimatedSprite2D, CharacterBody2D, CharacterBody2DVirtual, ProjectSettings};
use crate::player::player_state::PlayerState;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    pub (super) jump_velocity: f64,
    pub (super) gravity: f64,
    pub (super) jump_count_limit: u8,
    pub (super) jump_count: u8,
    #[export]
    pub player_floor_speed: f32,
    #[export]
    pub player_air_speed: f32,
    #[export]
    pub player_floor_velocity: f32,
    #[export]
    pub player_air_velocity: f32,
    #[export]
    pub player_stop_floor_velocity: f64,
    #[export]
    pub player_stop_air_velocity: f64,

    #[base]
    pub (super) base: Base<CharacterBody2D>,

    pub (super) sprite: Option<Gd<AnimatedSprite2D>>,

    pub (super) state: PlayerState,
}

#[godot_api]
impl Player {
    #[func]
    pub fn on_animation_finished(&mut self) {
        self.animation_finished();
    }
}

#[godot_api]
impl CharacterBody2DVirtual for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            jump_velocity: -300.0,
            gravity: ProjectSettings::singleton().get_setting(GodotString::from("physics/2d/default_gravity")).to(),
            jump_count_limit: 5,
            jump_count: 0,
            player_floor_speed: 200.0,
            player_air_speed: 200.0,
            player_floor_velocity: 10.0,
            player_air_velocity: 5.0,
            player_stop_floor_velocity: 600.0,
            player_stop_air_velocity: 300.0,
            base,
            sprite: None,
            state: PlayerState::Idle,
        }
    }

    fn process(&mut self, delta: f64) {
        match self.state {
            PlayerState::Idle => {
                self.state_idle_process(delta);
            }
            PlayerState::Run => {
                self.state_run_process(delta);
            }
            PlayerState::JumpUp => {
                self.state_air_process(delta);
            }
            PlayerState::Fall => {
                self.state_air_process(delta);
            }
            PlayerState::Attack => {
                self.state_attack_process(delta);
            }
        }

        self.animation_process(delta);
    }

    fn ready(&mut self) {
        self.sprite = Some(self.base.get_node_as("sprite"));
        self.sprite.as_deref_mut().unwrap().play();
        let callable = self.base.callable("on_animation_finished");
        self.sprite.as_deref_mut().unwrap().connect(StringName::from("animation_finished"), callable);
    }
}