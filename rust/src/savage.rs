use gdnative::api::*;
use gdnative::prelude::*;

use crate::bloo::Bloo;
use crate::entity::Entity;
use crate::enums::State;
use crate::runtime_data::RuntimeData;
use crate::traits::*;
use lerp::num_traits::clamp;
use lerp::Lerp;
use std::f64::consts::PI;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[register_with(Self::register_builder)]
pub struct Savage {
    entity: Entity,
    #[property]
    edge_detection_path: NodePath,
    edge_detection: Option<Ref<RayCast2D>>,
    #[property]
    wall_detection_path: NodePath,
    wall_detection: Option<Ref<RayCast2D>>,
    #[property]
    field_of_view: f32,
    #[property]
    deal_damage_to_bloo: f32,
    #[property]
    damage_velocity: Vector2,
    #[property]
    hit_bounce_power: Vector2,
    hurt_bloo: bool,
    can_jump: bool,
}

#[methods]
impl Savage {
    fn register_builder(_builder: &ClassBuilder<Self>) {}

    fn new(_owner: &KinematicBody2D) -> Self {
        Self {
            entity: Entity {
                health: 0.0,
                max_health: 100.0,
                speed: 250.0,
                velocity: Vector2::zero(),
                gravity: 30.0,
                jump_power: 1200.0,
                animation_path: NodePath::new(&GodotString::from("")),
                animation_player: None,
                friction: 0.05,
                facing_direction: Vector2::new(1.0, 0.0),
                runtime_data: RuntimeData::new(),
                global: None,
            },
            edge_detection_path: NodePath::new(&GodotString::from("")),
            edge_detection: None,
            wall_detection_path: NodePath::new(&GodotString::from("")),
            wall_detection: None,
            field_of_view: 45.0,
            deal_damage_to_bloo: 2.0,
            damage_velocity: Vector2::new(-1000.0, -400.0),
            hit_bounce_power: Vector2::new(600.0, -200.0),
            hurt_bloo: false,
            can_jump: false,
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: &KinematicBody2D) {
        self.entity._ready(owner);
        self.entity.animation_path = NodePath::new(&GodotString::from("Body/SavageAnimation"));
        self.wall_detection = Some(
            owner
                .get_node_as::<RayCast2D>(self.wall_detection_path.to_string().as_str())
                .unwrap()
                .assume_shared(),
        );

        self.edge_detection = Some(
            owner
                .get_node_as::<RayCast2D>(self.edge_detection_path.to_string().as_str())
                .unwrap()
                .assume_shared(),
        );
        self.entity.animation_player = Some(
            owner
                .get_node_as::<AnimationPlayer>(self.entity.animation_path.to_string().as_str())
                .unwrap()
                .assume_shared(),
        );

        self.entity.global = owner
            .get_node_as::<Node>("/root/RustGlobals")
            .unwrap()
            .assume_shared()
            .assume_safe()
            .get_child(0);
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f64) {
        self.gravity(owner);
        self.jump(owner);
        unsafe {
            self.hurt_bloo(owner);
        }
        self.movement(owner);
    }

    fn jump(&mut self, owner: &KinematicBody2D) {
        if self.can_jump && owner.is_on_floor() {
            self.entity.velocity.y -= self.entity.jump_power;
            self.change_state(owner, State::JUMP);
            self.can_jump = false;
        }
    }

    unsafe fn hurt_bloo(&mut self, owner: &KinematicBody2D) {
        for i in 0..owner.get_slide_count() {
            let collision = owner.get_slide_collision(i);
            if collision.is_some() {
                if let Some(is_kenimatic) = collision
                    .unwrap()
                    .assume_safe()
                    .collider()
                    .unwrap()
                    .assume_safe()
                    .cast::<KinematicBody2D>()
                {
                    if let Some(is_bloo) = is_kenimatic.cast_instance::<Bloo>() {
                        self.hurt_bloo = true;
                        is_bloo.base().call(
                            "apply_damage",
                            &[
                                self.deal_damage_to_bloo.to_variant(),
                                self.entity.facing_direction.to_variant(),
                                self.damage_velocity.to_variant(),
                            ],
                        );
                        if let Some(timer) = owner.get_node_as::<Timer>("HitBlooDelay") {
                            timer.start(-1.0);
                        } else {
                            godot_print!("savage has no timer named that !");
                        }

                        self.entity.velocity = Vector2::zero();
                        self.entity.velocity = Vector2::new(
                            self.hit_bounce_power.x * -self.entity.facing_direction.x,
                            self.hit_bounce_power.y,
                        );
                    }
                };
            }
        }
    }
    #[export]
    fn _on_visibility_screen_entered(&self, owner: &KinematicBody2D) {
        owner.set_physics_process(true);
        owner.set_process(true);
    }
    #[export]
    fn _on_visibility_screen_exited(&self, owner: &KinematicBody2D) {
        owner.set_physics_process(false);
        owner.set_process(false);
    }
    #[export]
    fn _on_jump_detection_body_entered(&mut self, _owner: &KinematicBody2D, _body: Ref<Object>) {
        self.can_jump = true;
    }
    #[export]
    fn _on_hit_bloo_delay_timeout(&mut self, _owner: &KinematicBody2D) {
        self.hurt_bloo = false;
    }
}

impl Movement for Savage {
    fn movement(&mut self, owner: &KinematicBody2D) {
        unsafe {
            if !self.hurt_bloo {
                if !self.edge_detection.unwrap().assume_safe().is_colliding()
                    || self.wall_detection.unwrap().assume_safe().is_colliding()
                {
                    if owner.is_on_floor() {
                        self.entity.velocity.x =
                            self.entity.velocity.x.lerp(0.0, self.entity.friction);

                        if self.entity.facing_direction == Vector2::new(1.0, 0.0) {
                            self.entity.flip_body(owner, false);
                        } else {
                            self.entity.flip_body(owner, true);
                        }
                    } else {
                        self.entity.velocity.x = self.entity.speed * self.entity.facing_direction.x
                    }
                } else {
                    if self.entity.runtime_data.current_state != State::ATTACK {
                        self.change_state(owner, State::MOVE);
                        self.entity.velocity.x = self.entity.speed * self.entity.facing_direction.x;
                    } else {
                        self.entity.velocity.x =
                            self.entity.velocity.x.lerp(0.0, self.entity.friction);
                    }
                }
            }
        }
        owner.move_and_slide(
            self.entity.velocity,
            Vector2::new(0.0, -1.0),
            false,
            4,
            PI / 4.0,
            false,
        );
    }
}

impl Gravity for Savage {
    fn gravity(&mut self, _owner: &KinematicBody2D) {
        self.entity.gravity(_owner);
    }
}

impl ChangeState for Savage {
    fn change_state(&mut self, _owner: &KinematicBody2D, state: State) {
        self.entity.change_state(_owner, state);
    }
}
