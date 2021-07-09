use gdnative::api::*;
use gdnative::prelude::*;


#[derive(NativeClass)]
#[inherit(RigidBody2D)]
#[derive(ToVariant, FromVariant)]
#[register_with(Self::register_builder)]
pub struct Bullet {
    #[property]
	deal_damage_to_bloo: f32,
    #[property]
    max_bounces: u16,
    free_bullet_timer: Option<Ref<Timer>>,
    bounces: u16,
    facing_direction: u8,
    damage_velocity: Vector2,

	
	// [Export] private int _maxBounces = 10;
	// private Timer FreeBullet;
	// private int bounces = 0;
	// public Vector2 facingDirection = Vector2.Right;
	// public Vector2 damageVelocity = Vector2.Zero;
}

#[methods]
impl Bullet {

    fn register_builder(_builder: &ClassBuilder<Self>) {

    }

    fn new(_owner : &RigidBody2D) -> Self {
        Self {
            deal_damage_to_bloo: 15.0,
            max_bounces: 10,
            free_bullet_timer: None,
            bounces: 0,
            facing_direction: 1,
            damage_velocity: Vector2::zero(),
        }
    }

    #[export]
    unsafe fn _ready(&mut self, _owner : &RigidBody2D) {

        self.free_bullet_timer = Some(
            _owner  
                .get_node_as::<Timer>("FreeBullet")
                .unwrap()
                .assume_shared());


    }

    #[export]
    unsafe fn _integrate_forces(&mut self, _owner : &RigidBody2D, _state: Variant) {
        _owner.set_rotation(_owner.linear_velocity().angle_from_x_axis().get().into());
        
    }

    #[export]
    fn fire(&self, owner: &RigidBody2D, force: Vector2, facingdirection: Vector2)
	{
		owner.set_scale(facingdirection);
		owner.apply_impulse(Vector2::zero(), force);
	}
 
    

}