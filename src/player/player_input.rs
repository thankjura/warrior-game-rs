use godot::prelude::StringName;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref PLAYER_JUMP: StringName = StringName::from("ui_up");
    pub static ref PLAYER_LEFT: StringName = StringName::from("ui_left");
    pub static ref PLAYER_RIGHT: StringName = StringName::from("ui_right");
    pub static ref PLAYER_ATTACK: StringName = StringName::from("ui_attack");
}