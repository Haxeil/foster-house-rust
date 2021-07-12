use crate::enums::State;
use gdnative::prelude::*;
pub trait FlipBody {
    fn flip_body(&mut self, _owner: &KinematicBody2D, flip_h: bool);
}
pub trait SetHealth {
    fn set_health(&mut self, _owner: &KinematicBody2D, value: f32);
}
pub trait Gravity {
    fn gravity(&mut self, _owner: &KinematicBody2D);
}

pub trait Movement {
    fn movement(&mut self, _owner: &KinematicBody2D);
}

pub trait Jump {
    fn jump(&mut self, _owner: &KinematicBody2D);
}
pub trait PlayAnimation {
    fn play_animation(&mut self, _owner: &KinematicBody2D);
}

pub trait ChangeState {
    fn change_state(&mut self, _owner: &KinematicBody2D, state: State);
}

pub trait Die {
    fn die(&mut self, _owner: &KinematicBody2D);
}

pub trait ShootingInrangeTarget {
    unsafe fn shooting_in_range_target(&mut self, _owner: &KinematicBody2D);
}

pub trait ApplyDamage {
    fn apply_damage(
        &mut self,
        owner: &KinematicBody2D,
        amount: &f32,
        enemy_direction: Vector2,
        damage_velocity: Vector2,
    );
}
