use gdnative::api::*;
//use gdnative::nativescript::property::IntHint;
use gdnative::prelude::*;
//use lerp::Lerp;
use lerp::num_traits::clamp;
//use std::f64::consts::PI;
use gdnative::nativescript::{Export, Instance};
//use gdnative::nativescript::init::property::RangeHint;
use crate::enums::State;
use crate::runtime_data::RuntimeData;
use crate::traits::{ChangeState, FlipBody};
use crate::game_manager::GameManager;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[register_with(Self::register_builder)]
#[derive(ToVariant, FromVariant)]
pub struct Entity {
    pub health: f32,
    #[property(default = 100.0)]
    pub max_health: f32,
    #[property(default = 300.0)]
    pub speed: f32,
    pub velocity: Vector2,
    #[property(default = 35.0)]
    pub gravity: f32,
    #[property(default = 1000.0)]
    pub jump_power: f32,
    #[property]
    pub animation_path: NodePath,
    pub animation_player: Option<Ref<AnimationPlayer>>,

    #[property(default = 0.25)]
    pub friction: f32,
    #[property]
    pub facing_direction: Vector2,
    pub runtime_data: RuntimeData,
    pub global : Option<Ref<Node>>,
}

#[methods]
impl Entity {
    fn register_builder(_builder: &ClassBuilder<Self>) {}
    /// The "constructor" of the class.
    fn new(_owner: &KinematicBody2D) -> Self {
        Entity {
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
        }
    }

    #[export]
    pub unsafe fn _ready(&mut self, _owner : &KinematicBody2D)
    {

        
  
        // self.animation_player = Some(_owner.get_node_as::<AnimationPlayer>
        //     (self.animation_path.to_string().as_str()).unwrap().assume_shared());

        // self.sprite_animation = Some(_owner.get_node_as::<AnimatedSprite>
        //     (self.sprite_animation_path.to_string().as_str()).unwrap().assume_shared());
    }
}

impl FlipBody for Entity {
    // fn play_animation(&mut self, _owner : &KinematicBody2D,
    //     animation_player : &TRef<AnimationPlayer>,
    //     sprite_animation : &TRef<AnimatedSprite>)
    // {

    //     let update_animation = |anim : &str|
    //     {

    //         animation_player.play(anim, -1.0, 1.0, false);
    //         sprite_animation.play(anim, false);

    //     };

    //     match self.runtime_data.current_state
    //     {
    //         State::IDLE => {update_animation("Idle")},
    //         State::MOVE => {update_animation("Move")},
    //         State::JUMP => {update_animation("Jump")},
    //         State::FALL => {update_animation("Fall")},
    //         State::DEAD => {update_animation("Die")},
    //         State::HURT => {update_animation("Hurt")},
    //     }

    // }
    fn flip_body(&mut self, _owner: &KinematicBody2D, flip_h: bool) {
        let flip = |facing: &mut f32| {
            _owner.apply_scale(Vector2::new(-1.0, 1.0));
            *facing *= -1.0;
        };

        if flip_h && self.facing_direction.x == -1.0 {
            flip(&mut self.facing_direction.x);
        } else if !flip_h && self.facing_direction.x == 1.0 {
            flip(&mut self.facing_direction.x);
        }
    }
}

impl ChangeState for Entity {
    fn change_state(&mut self, _owner: &KinematicBody2D, state: State) {
        if self.runtime_data.current_state == State::DEAD {
            return;
        }

        self.runtime_data.change_next_state(state);

        if self.runtime_data.next_state != self.runtime_data.current_state {
            self.runtime_data
                .change_prev_state(self.runtime_data.current_state);
        }
        self.runtime_data
            .change_current_state(self.runtime_data.next_state);

        //self.play_animation(_owner);
    }
}
