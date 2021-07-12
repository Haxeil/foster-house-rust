use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_builder)]

pub struct GameManager {
    #[property(default = 20)]
    pub coin_counter: u32,
    #[property]
    pub bloo_global_position: Vector2,
    //pub bloo_runtime_data : BlooRuntimeData,
}

#[methods]
impl GameManager {
    fn register_builder(_builder: &ClassBuilder<Self>) {
        _builder.add_signal(Signal {
            name: "coinPicked",
            args: &[],
        });

        _builder.add_signal(Signal {
            name: "get_bloo_gb_position",
            args: &[],
        });
    }

    fn new(_owner: &Node) -> Self {
        GameManager {
            coin_counter: 20,
            bloo_global_position: Vector2::zero(),
        }
    }
    #[export]
    pub fn get_bloo_gb_position(&self, _owner: &Node) -> Vector2 {
        self.bloo_global_position
    }

    #[export]
    unsafe fn _ready(&mut self, _owner: TRef<Node>) {

        // _owner.connect("change_state", _owner, "change_state", VariantArray::new_shared(), 0).unwrap();
    }
}
