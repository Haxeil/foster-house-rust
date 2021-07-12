use crate::bloo::Bloo;
use crate::entity::Entity;
use crate::enums::State;
use crate::runtime_data::RuntimeData;
use crate::traits::{
    ChangeState, FlipBody, Gravity, Movement, PlayAnimation, SetHealth, ShootingInrangeTarget,
};
use gdnative::api::*;
use gdnative::prelude::*;
use lerp::num_traits::clamp;
use lerp::num_traits::ToPrimitive;
use lerp::Lerp;
use std::convert::TryInto;
//use std::any::Any;
//use std::convert::TryInto;
use std::f64::consts::PI;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[derive(ToVariant, FromVariant)]
#[register_with(Self::register_builder)]
pub struct Cheese {
    pub entity: Entity,

    bullet: Option<Ref<PackedScene>>,

    #[property]
    position_2d_path: NodePath,
    position_2d: Option<Ref<Position2D>>,
    bullet_delay: bool,
    #[property]
    launch_vector: Vector2,
    #[property]
    max_distence_range: f32,
    #[property]
    field_of_view: f32,
    #[property]
    deal_damage_to_bloo: f32,
    #[property]
    damage_velocity: Vector2,
    #[property]
    additional_force: f32,
    #[property]
    edge_detection_path: NodePath,
    edge_detection: Option<Ref<RayCast2D>>,
    #[property]
    wall_detection_path: NodePath,
    wall_detection: Option<Ref<RayCast2D>>,
    bloo_node: Option<Ref<KinematicBody2D>>,
    #[property]
    delay_timer_path: NodePath,
    delay_timer_bullet: Option<Ref<Timer>>,
}

#[methods]
impl Cheese {
    fn register_builder(_builder: &ClassBuilder<Self>) {}

    fn new(_owner: &KinematicBody2D) -> Self {
        Cheese {
            entity: Entity {
                health: 0.0,
                max_health: 100.0,
                speed: 200.0,
                velocity: Vector2::zero(),
                gravity: 35.0,
                jump_power: 0.0,
                animation_path: NodePath::new(&GodotString::from("")),
                animation_player: None,

                friction: 0.2,
                facing_direction: Vector2::new(1.0, 0.0),
                runtime_data: RuntimeData::new(),
                global: None,
            },
            bullet: None,
            position_2d: None,
            bullet_delay: true,
            launch_vector: Vector2::zero(),
            max_distence_range: 700.0,
            field_of_view: 45.0,
            deal_damage_to_bloo: 20.0,
            damage_velocity: Vector2::new(100.0, -200.0),
            additional_force: 2.1,
            edge_detection: None,
            wall_detection: None,
            position_2d_path: NodePath::new(&GodotString::from("")),
            edge_detection_path: NodePath::new(&GodotString::from("")),
            wall_detection_path: NodePath::new(&GodotString::from("")),
            bloo_node: None,
            delay_timer_path: NodePath::new(&GodotString::from("")),
            delay_timer_bullet: None,
        }
    }

    #[export]
    unsafe fn _ready(&mut self, _owner: &KinematicBody2D) {
        self.entity._ready(_owner);
        self.entity.animation_path = NodePath::new(&GodotString::from("Body/CheeseAnimation"));

        self.position_2d = Some(
            _owner
                .get_node_as::<Position2D>(self.position_2d_path.to_string().as_str())
                .unwrap()
                .assume_shared(),
        );

        self.wall_detection = Some(
            _owner
                .get_node_as::<RayCast2D>(self.wall_detection_path.to_string().as_str())
                .unwrap()
                .assume_shared(),
        );

        self.edge_detection = Some(
            _owner
                .get_node_as::<RayCast2D>(self.edge_detection_path.to_string().as_str())
                .unwrap()
                .assume_shared(),
        );

        self.delay_timer_bullet = Some(
            _owner
                .get_node_as::<Timer>(self.delay_timer_path.to_string().as_str())
                .unwrap()
                .assume_shared(),
        );

        self.entity.animation_player = Some(
            _owner
                .get_node_as::<AnimationPlayer>(self.entity.animation_path.to_string().as_str())
                .unwrap()
                .assume_shared(),
        );

        self.bullet = Some(
            ResourceLoader::godot_singleton()
                .load(
                    GodotString::from_str("res://scenes/Bullet.tscn"),
                    GodotString::from_str("PackedScene"),
                    false,
                )
                .unwrap()
                .assume_safe()
                .cast::<PackedScene>()
                .unwrap()
                .assume_shared(),
        );

        self.bloo_node = Some(
            _owner
                .get_tree()
                .unwrap()
                .assume_safe()
                .root()
                .unwrap()
                .assume_safe()
                .get_node("/root")
                .unwrap()
                .assume_safe()
                .get_child(1)
                .unwrap()
                .assume_safe()
                .get_node("Bloo")
                .unwrap()
                .assume_safe()
                .cast::<KinematicBody2D>()
                .unwrap()
                .assume_shared(),
        );

        self.entity.global = _owner
            .get_node_as::<Node>("/root/RustGlobals")
            .unwrap()
            .assume_shared()
            .assume_safe()
            .get_child(0);

        let angle = Angle::radians(self.position_2d.unwrap().assume_safe().rotation() as f32);

        self.entity.facing_direction = self.entity.facing_direction.rotated(angle);
        self._on_visibility_screen_exited(_owner);
    }

    #[export]
    unsafe fn _process(&mut self, _owner: &KinematicBody2D, _delta: f64) {
        self.shooting_in_range_target(_owner);
    }

    #[export]
    fn _physics_process(&mut self, _owner: &KinematicBody2D, _delta: f64) {
        self.gravity(_owner);
        self.movement(_owner);
    }

    fn bloo_in_range(&self, distance_to_bloo: f32) -> bool {
        distance_to_bloo <= self.max_distence_range
    }
    fn bloo_in_vision(&self, cheese_to_bloo_angle: f32) -> bool {
        cheese_to_bloo_angle.abs() <= self.field_of_view.to_radians()
    }

    #[export]
    fn _on_delay_time_out(&mut self, _owner: &KinematicBody2D) {
        self.bullet_delay = true;
    }

    #[export]
    unsafe fn send_bullet(&mut self, _owner: &KinematicBody2D) {
        let bullet = self
            .bullet
            .as_ref()
            .unwrap()
            .assume_safe()
            .instance(0)
            .unwrap()
            .assume_safe()
            .cast::<RigidBody2D>()
            .unwrap();

        bullet.set_global_position(self.position_2d.unwrap().assume_safe().global_position());
        bullet.set("damage_velocity", self.damage_velocity);
        bullet.set("facing_direction", self.entity.facing_direction);

        bullet.call(
            "fire",
            &[
                self.launch_vector.to_variant(),
                Vector2::new(self.entity.facing_direction.x, 0.0).to_variant(),
            ],
        );

        _owner
            .get_parent()
            .unwrap()
            .assume_safe()
            .add_child(bullet, true);
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
    fn _on_behind_detection_body_entered(&mut self, owner: &KinematicBody2D, _body: Ref<Object>) {
        if self.entity.facing_direction == Vector2::new(1.0, 0.0) {
            self.entity.flip_body(owner, false);
        } else {
            self.entity.flip_body(owner, true);
        }
    }
}

impl Movement for Cheese {
    fn movement(&mut self, _owner: &KinematicBody2D) {
        unsafe {
            if !self.edge_detection.unwrap().assume_safe().is_colliding()
                || self.wall_detection.unwrap().assume_safe().is_colliding()
            {
                self.entity.velocity.x = self.entity.velocity.x.lerp(0.0, self.entity.friction);
                //Lerp(_velocity.x, 0, _friction);

                if self.entity.facing_direction.x == 1.0 {
                    self.entity.flip_body(_owner, false);
                } else {
                    self.entity.flip_body(_owner, true);
                }
            } else {
                if self.entity.runtime_data.current_state != State::ATTACK {
                    self.change_state(_owner, State::MOVE);
                    self.entity.velocity.x =
                        self.entity.speed * self.entity.facing_direction.x as f32;
                } else {
                    self.entity.velocity.x = self.entity.velocity.x.lerp(0.0, self.entity.friction);
                    //Mathf.Lerp(_velocity.x, 0, _friction);
                }
            }

            _owner.move_and_slide(
                self.entity.velocity,
                Vector2::new(0.0, -1.0),
                false,
                4,
                PI / 4.0,
                true,
            );
        }
    }
}

impl ChangeState for Cheese {
    fn change_state(&mut self, _owner: &KinematicBody2D, state: State) {
        self.entity.change_state(_owner, state);
        self.play_animation(_owner);
    }
}

impl PlayAnimation for Cheese {
    fn play_animation(&mut self, _owner: &KinematicBody2D) {
        let animation_player = if let Some(anim) = self.entity.animation_player {
            unsafe { Some(anim.assume_safe()) }
        } else {
            godot_print!("something went wrong with animation_player");
            return;
        };

        let update_animation = |anim: &str| {
            animation_player.unwrap().play(anim, -1.0, 1.0, false);
        };

        match self.entity.runtime_data.current_state {
            State::IDLE => update_animation("Idle"),
            State::MOVE => update_animation("Move"),
            State::JUMP => update_animation("Jump"),
            State::FALL => update_animation("Fall"),
            State::DEAD => update_animation("Die"),
            State::HURT => update_animation("Hurt"),
            State::ATTACK => update_animation("ATTACK"),
            _ => godot_print!("Animation {}", _owner.name()),
        }
    }
}

impl Gravity for Cheese {
    fn gravity(&mut self, _owner: &KinematicBody2D) {
        self.entity.gravity(_owner);
    }
}

impl ShootingInrangeTarget for Cheese {
    unsafe fn shooting_in_range_target(&mut self, _owner: &KinematicBody2D) {
        let space_state = Physics2DServer::godot_singleton()
            .space_get_direct_state(_owner.get_world_2d().unwrap().assume_safe().space())
            .unwrap()
            .assume_safe();

        let bloo_position = self
            .entity
            .global
            .unwrap()
            .assume_safe()
            .get("bloo_global_position")
            .to_vector2();

        let result = space_state.intersect_ray(
            _owner.global_position(),
            bloo_position,
            VariantArray::new_shared(),
            _owner.collision_mask(),
            true,
            false,
        );

        let mut is_bloo_collider = false;

        if let Some(is_kinematicbody_collider) =
            result.get("collider").try_to_object::<KinematicBody2D>()
        {
            if let Some(_collider) = is_kinematicbody_collider
                .assume_safe()
                .cast_instance::<Bloo>()
            {
                is_bloo_collider = true;
            }
        }

        if result.is_empty() || !is_bloo_collider {
            self.change_state(_owner, State::MOVE);
            return;
        }
        let launch_position = self.position_2d.unwrap().assume_safe().global_position();
        // var distenceToBloo = (_gameManager.blooPosition - _launchPosition.GlobalPosition).Length();
        // var directionToBloo = (_gameManager.blooPosition - _launchPosition.GlobalPosition);
        // var cheeseToBlooAngle = facingDirection.AngleTo(directionToBloo);
        let distance_to_bloo = (bloo_position - launch_position).length();

        let direction_to_bloo = bloo_position - launch_position;

        let cheese_to_bloo_angle = self
            .entity
            .facing_direction
            .angle_to(direction_to_bloo)
            .get();

        if self.bloo_in_range(distance_to_bloo) && self.bloo_in_vision(cheese_to_bloo_angle) {
            if result
                .get("position")
                .try_to_vector2()
                .unwrap_or(Vector2::zero())
                != Vector2::zero()
            {
                if self.bullet_delay {
                    self.launch_vector = Vector2::new(
                        (cheese_to_bloo_angle * self.entity.facing_direction.x).cos()
                            * self.entity.facing_direction.x as f32,
                        (cheese_to_bloo_angle * self.entity.facing_direction.x).sin(),
                    ) * distance_to_bloo
                        * self.additional_force;

                    self.change_state(_owner, State::ATTACK);

                    self.delay_timer_bullet.unwrap().assume_safe().start(-1.0);

                    self.bullet_delay = false;
                }
            } else {
                self.change_state(_owner, State::MOVE);
            }
        } else {
            self.change_state(_owner, State::MOVE);
        }
    }
}
