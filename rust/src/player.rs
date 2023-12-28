use godot::prelude::*;
use godot::engine::{ICharacterBody2D, CharacterBody2D};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    speed: f64,

    #[base]
    base: Base<CharacterBody2D>,
}
impl Player {
    fn get_input(&self) -> Vector2 {
        let input_direction: Vector2 = Input::singleton().get_vector(StringName::from("ui_left"), StringName::from("ui_right"), StringName::from("ui_up"), StringName::from("ui_down"));
        input_direction
    }
}

#[godot_api]
pub impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Hello, world!");

        Self {
            speed: 100.0,
            base,
        }
    }

    // set velocity based on speed and user input to get direction
    fn physics_process(&mut self, _delta: f64) {
        let velocity: Vector2 = self.get_input() * self.speed as f32;

        self.base.set_velocity(velocity);
        self.base.move_and_slide();

    }
}

