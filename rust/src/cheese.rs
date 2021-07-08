use gdnative::prelude::*;
use gdnative::api::*;
use crate::bloo::Bloo;
use crate::entity::Entity;
use crate::runtime_data::RuntimeData;
use crate::traits::{Movement, FlipBody, SetHealth, PlayAnimation, ChangeState, ShootingInrangeTarget};
use lerp::num_traits::clamp;
use lerp::Lerp;
use crate::enums::State;
use std::any::Any;
use std::convert::TryInto;
use std::f64::consts::PI;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[derive(ToVariant, FromVariant)]
#[register_with(Self::register_builder)]
pub struct Cheese
{

    entity : Entity,

    #[property]
    position_2d_path : NodePath,
    position_2d : Option<Ref<Position2D>>,
    bullet_delay : bool,
    #[property]
    launch_vector : Vector2,
    #[property]
    max_distence_range : f32,
    #[property]
    field_of_view : f32,
    #[property]
    deal_damage_to_bloo : f32,
    #[property]
    damage_velocity : Vector2,
    additional_force : f32,
    #[property]
    edge_detection_path : NodePath,
    edge_detection : Option<Ref<RayCast2D>>,
    #[property]
    wall_detection_path : NodePath,
    wall_detection : Option<Ref<RayCast2D>>,
    bloo_node : Option<Ref<KinematicBody2D>>,
    

}


#[methods]
impl Cheese 
{
    fn register_builder(_builder: &ClassBuilder<Self>) 
    {

    }

    
    fn new(_owner : &KinematicBody2D) -> Self
    {

        Cheese
        {
            entity: Entity
            {
                health : 0.0,
                max_health : 100.0,
                speed : 200.0,
                velocity : Vector2::zero(),
                gravity : 35.0,
                jump_power : 0.0,
                animation_path : NodePath::new(&GodotString::from("")),
                animation_player : None,

                friction : 0.2,
                facingdirection : 1,
                runtime_data : RuntimeData::new(),
                global : None,
            },
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
            bloo_node : None,
            

  
        }
        
    }
    
    #[export]
    unsafe fn _ready(&mut self, _owner : &KinematicBody2D)
    {
        self.entity.animation_path = NodePath::new(&GodotString::from("Body/CheeseAnimation"));

        self.position_2d = Some(_owner.get_node_as::<Position2D>
            (self.position_2d_path.to_string().as_str()).unwrap().assume_shared());

        self.wall_detection = Some(_owner.get_node_as::<RayCast2D>
            (self.wall_detection_path.to_string().as_str()).unwrap().assume_shared());

        self.edge_detection = Some(_owner.get_node_as::<RayCast2D>
            (self.edge_detection_path.to_string().as_str()).unwrap().assume_shared());

        self.entity.animation_player = Some(_owner.get_node_as::<AnimationPlayer>
            (self.entity.animation_path.to_string().as_str()).unwrap().assume_shared());

        self.bloo_node = Some(
            _owner.get_tree().unwrap().assume_safe()
                .root().unwrap().assume_safe()
                .get_node("/root").unwrap().assume_safe()
                .get_child(1).unwrap().assume_safe()
                .get_node("Bloo").unwrap().assume_safe()
                .cast::<KinematicBody2D>().unwrap().assume_shared()
        );
        
        self.entity.global = _owner.get_node_as::<Node>("/root/RustGlobals")
            .unwrap()
            .assume_shared()
            .assume_safe().get_child(0);

        
    }

    #[export]
    unsafe fn _process(&mut self, _owner : &KinematicBody2D, _delta : f64)
    {
        
    }

    #[export]
    unsafe fn _physics_process(&mut self, _owner : &KinematicBody2D, _delta : f64)
    {
        self.shooting_in_range_target(_owner);
        self.movement(_owner);
    }

}

impl Movement for Cheese
{
    fn movement(&mut self, _owner : &KinematicBody2D) {
        unsafe {
            if !self.edge_detection.unwrap().assume_safe().is_colliding() || 
                self.wall_detection.unwrap().assume_safe().is_colliding()
                
            {
                self.entity.velocity.x = self.entity.velocity.x.lerp
                                         (0.0, self.entity.friction);
                //Lerp(_velocity.x, 0, _friction);

                if self.entity.facingdirection == 1
                {
                    self.entity.flip_body(_owner, false);
                }
                else
                {
                    self.entity.flip_body(_owner, true);
                }

            }
            else
            {
                if self.entity.runtime_data.current_state != State::ATTACK
                {
                    self.change_state(_owner, State::MOVE);
                    self.entity.velocity.x = self.entity.speed * self.entity.facingdirection as f32;
                }
                else
                {
                    
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

impl ChangeState for Cheese
{
    fn change_state(&mut self, _owner: &KinematicBody2D, state: State) {
        self.entity.change_state(_owner, state);
        self.play_animation(_owner);
        
    }
}

impl PlayAnimation for Cheese
{
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

impl ShootingInrangeTarget for Cheese
{
    unsafe fn shooting_in_range_target(&mut self, _owner : &KinematicBody2D) {



        let space_state = 
        Physics2DServer::godot_singleton().space_get_direct_state(
            _owner.get_world_2d()
            .unwrap()
            .assume_safe()
            .space()
        ).unwrap().assume_safe();

        
        let bloo_position = self.entity.global
            .unwrap()
            .assume_safe()
            .get("bloo_global_position").to_vector2();

        let result = space_state.intersect_ray(
            _owner.global_position(),
            bloo_position, 
            VariantArray::new_shared(), 
            _owner.collision_mask(),
            true, 
            false,
        );




        let collider = result.get("collider_id");
        
        
        godot_print!("{:?}, {:?}, {:?}",
            collider, 
            bloo_position,
            self.bloo_node.unwrap().assume_safe());

        if result.is_empty() || result.get("count") == 0.to_variant()
        {
            
        }
            


        

        /*
        
		var spaceState = GetWorld2d().DirectSpaceState;
		var result = spaceState.IntersectRay(this.GlobalPosition, _gameManager.blooPosition, new Godot.Collections.Array { this }, this.CollisionMask);
		if (result == null || result.Count == 0 || !(result["collider"] is Bloo))
		{
			EmitSignal("StateChanged", Enums.GamePlayState.MOVING);
			return;
		}

		var distenceToBloo = (_gameManager.blooPosition - _launchPosition.GlobalPosition).Length();
		var directionToBloo = (_gameManager.blooPosition - _launchPosition.GlobalPosition);
		var cheeseToBlooAngle = facingDirection.AngleTo(directionToBloo);
		if (!(_BlooInRange(distenceToBloo) || _BlooInVision(cheeseToBlooAngle)))
		{
			EmitSignal("StateChanged", Enums.GamePlayState.MOVING);

		}
		else if(_BlooInRange(distenceToBloo) && _BlooInVision(cheeseToBlooAngle))
		{
			if (result["position"] is Vector2 position)
			{
				if (_bulletDelay)
				{
					_launchVector = new Vector2(Mathf.Cos(cheeseToBlooAngle * facingDirection.x) * facingDirection.x, Mathf.Sin(cheeseToBlooAngle * facingDirection.x)) * distenceToBloo * _applyMoreForce;
					EmitSignal("StateChanged", Enums.GamePlayState.ATTACKING);
					_delayTimerBullet.Start();
					_bulletDelay = false;
				}
			}
			else
			{
				EmitSignal("StateChanged", Enums.GamePlayState.MOVING);
			}
		}
		else
		{
			EmitSignal("StateChanged", Enums.GamePlayState.MOVING);
		}

			
        
        */
    }
}