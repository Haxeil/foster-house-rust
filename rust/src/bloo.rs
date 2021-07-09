use gdnative::api::*;
use gdnative::prelude::*;
use lerp::num_traits::clamp;
use lerp::Lerp;
use std::f64::consts::PI;

use crate::entity::Entity;
use crate::enums::State;
use crate::runtime_data::RuntimeData;
use crate::traits::*;

/// The Game "class"
#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[register_with(Self::register_builder)]
pub struct Bloo {
    //#[property]
    // can't export "Inherited properties
    //#[register_with(int_entity)]
    entity: Entity,
    #[property]
    sprite_animation_path: NodePath,
    sprite_animation: Option<Ref<AnimatedSprite>>,
}

#[methods]
impl Bloo {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {
        _builder.add_signal(Signal {
            name: "health_updated",
            args: &[],
        });
        _builder.add_signal(Signal {
            name: "max_health_updated",
            args: &[],
        });
    }

    /// The "constructor" of the class.
    fn new(_owner: &KinematicBody2D) -> Self {
        Bloo {
            entity: Entity {
                
                max_health: 100.0,
                health: 0.0,
                speed: 300.0,
                velocity: Vector2::new(0.0, 0.0),
                gravity: 35.0,
                jump_power: 1000.0,
                animation_path: NodePath::new(&GodotString::from("Animatio")),
                animation_player: None,

                friction: 0.25,
                facing_direction: Vector2::new(1.0, 0.0),
                runtime_data: RuntimeData::new(),
                global : None,
            },
            sprite_animation_path: NodePath::new(&GodotString::from("Torso")),
            sprite_animation: None,
        }
    }

    // In order to make a method known to Godot, the #[export] attribute has to be used.
    // In Godot script-classes do not actually inherit the parent class.
    // Instead they are "attached" to the parent object, called the "owner".
    // The owner is passed to every single exposed method.
    #[export]
    unsafe fn _ready(&mut self, _owner: &KinematicBody2D) {
        self.entity.animation_path = NodePath::new(&GodotString::from("AnimationPlayer"));
        self.sprite_animation_path = NodePath::new(&GodotString::from("Body/Torso"));

        self.entity.health = self.entity.max_health;

        _owner.emit_signal("max_health_updated", &[self.entity.max_health.to_variant()]);

        self.change_state(_owner, State::IDLE);

        self.entity.animation_player = Some(
            _owner
                .get_node_as::<AnimationPlayer>(self.entity.animation_path.to_string().as_str())
                .unwrap()
                .assume_shared(),
        );

        self.sprite_animation = Some(
            _owner
                .get_node_as::<AnimatedSprite>(self.sprite_animation_path.to_string().as_str())
                .unwrap()
                .assume_shared(),
        );
        self.entity.global = _owner.get_node_as::<Node>("/root/RustGlobals")
        .unwrap().assume_shared().assume_safe().get_child(0);


        self.entity._ready(_owner);
    }

    #[export]
    unsafe fn _physics_process(&mut self, _owner: &KinematicBody2D, _delta: f64) {
        self.gravity(_owner);
        self.jump(_owner);
        self.movement(_owner);
    }
}

impl Jump for Bloo {
    fn jump(&mut self, _owner: &KinematicBody2D) {
        //owner.get_node_as::<Timer>("start_timer").unwrap())

        let cayote_timer = unsafe { _owner.get_node_as::<Timer>("CayoteTimer").unwrap() };

        if Input::godot_singleton().is_action_pressed("jump")
            && (_owner.is_on_floor() || !cayote_timer.is_stopped())
        {
            //apply_damage(_owner, self, &10.0, -1.0, Vector2::new(240.0, 300.0));
            cayote_timer.stop();
            self.entity.velocity.y = -self.entity.jump_power;
            self.change_state(_owner, State::JUMP);
        }
    }
}

impl PlayAnimation for Bloo {
    fn play_animation(&mut self, _owner: &KinematicBody2D) {
        let animation_player = if let Some(anim) = self.entity.animation_player {
            unsafe { Some(anim.assume_safe()) }
        } else {
            godot_print!("something went wrong with animation_player");
            return;
        };

        let sprite_animation = if let Some(anim) = self.sprite_animation {
            unsafe { Some(anim.assume_safe()) }
        } else {
            godot_print!("something went wrong with sprite_animation");
            return;
        };

        let update_animation = |anim: &str| {
            animation_player.unwrap().play(anim, -1.0, 1.0, false);
            sprite_animation.unwrap().play(anim, false);
        };

        match self.entity.runtime_data.current_state {
            State::IDLE => update_animation("Idle"),
            State::MOVE => update_animation("Move"),
            State::JUMP => update_animation("Jump"),
            State::FALL => update_animation("Fall"),
            State::DEAD => update_animation("Die"),
            State::HURT => update_animation("Hurt"),
            _ => godot_print!("Bloo {}", "State"),
        }
    }
}

impl ChangeState for Bloo {
    fn change_state(&mut self, _owner: &KinematicBody2D, state: State) {
        self.entity.change_state(_owner, state);
        self.play_animation(_owner)
    }
}

impl Gravity for Bloo {
    fn gravity(&mut self, _owner: &KinematicBody2D) {
        if _owner.is_on_ceiling() {
            self.entity.velocity.y = self.entity.gravity;
        }

        if _owner.is_on_floor() {
            self.entity.velocity.y = 0.0;
        } else {
            self.entity.velocity.y += self.entity.gravity;

            if self.entity.velocity.y != 0.0
                && self.entity.velocity.y > self.entity.gravity
                && self.entity.velocity.y > -self.entity.jump_power / self.entity.gravity
            {
                //unsafe { change_state(_owner, State::Bloo(BlooState::FALL), bloo) }

                self.change_state(_owner, State::FALL);
            }
        }
    }
}

impl Movement for Bloo {
    fn movement(&mut self, _owner: &KinematicBody2D) {
        let input = Input::godot_singleton();

        // let space_state = World2D::new();

        // let space_state = space_state.direct_space_state().unwrap();

        // unsafe
        // {

        //     let result = space_state.assume_safe().intersect_ray(Vector2::new(0.0, 0.0),
        //     Vector2::new(100.0, 0.0), VariantArray::new_shared(), _owner.collision_layer(),
        //     true, true);
        //     godot_print!("{:?}", result);
        // }

        if input.is_action_pressed("right") {
            self.entity.velocity.x = self.entity.speed;
            self.change_state(_owner, State::MOVE);
            self.entity.flip_body(_owner, true);
        } else if input.is_action_pressed("left") {
            self.entity.velocity.x = -self.entity.speed;
            self.change_state(_owner, State::MOVE);
            self.entity.flip_body(_owner, false);
        } else {
            self.entity.velocity.x = self.entity.velocity.x.lerp(0.0, self.entity.friction);
            if _owner.is_on_floor() {
                self.change_state(_owner, State::IDLE);
            }
        }

        let was_on_floor = _owner.is_on_floor();

        _owner.move_and_slide(
            self.entity.velocity,
            Vector2::new(0.0, -1.0),
            false,
            4,
            PI / 4.0,
            true,
        );

        if !_owner.is_on_floor() && was_on_floor {
            let cayote_timer = unsafe { _owner.get_node_as::<Timer>("CayoteTimer").unwrap() };
            cayote_timer.start(-1.0);
        }

        unsafe
        {
            self.entity.global
                .unwrap()
                .assume_safe()
                .set("bloo_global_position", _owner.global_position());
        }
    }
}

impl Die for Bloo {
    fn die(&mut self, _owner: &KinematicBody2D) {
        _owner.queue_free();
    }
}

fn apply_damage(
    _owner: &KinematicBody2D,
    bloo: &mut Bloo,
    amount: &f32,
    enemy_direction: f32,
    damage_velocity: Vector2,
) {
    bloo.change_state(_owner, State::HURT);
    bloo.set_health(_owner, bloo.entity.health - *amount);
    bloo.entity.velocity = Vector2::zero();
    //+= Vector2(damageVelocity.x * enemyDirection.x, damageVelocity.y);
    bloo.entity.velocity += Vector2::new(damage_velocity.x * enemy_direction, damage_velocity.y);
}

impl SetHealth for Bloo {
    fn set_health(&mut self, _owner: &KinematicBody2D, value: f32) {
        let prev_health = self.entity.health;

        self.entity.health = clamp(value, 0.0, self.entity.max_health);

        if prev_health != self.entity.health {
            _owner.emit_signal("health_updated", &[self.entity.health.to_variant()]);

            if self.entity.health == 0.0 {
                self.die(_owner);
            }
        }
    }
}
