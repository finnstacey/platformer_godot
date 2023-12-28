use godot::prelude::*;
use godot::engine::Sprite2D;
use godot::engine::ISprite2D;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Player {
    speed: f64,
    angular_speed: f64,

    #[base]
    sprite: Base<Sprite2D>,
}

#[godot_api]
pub impl ISprite2D for Player {
    fn init(sprite: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!");

        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            sprite,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.sprite.rotate((self.angular_speed * delta) as f32);

        let rotation: f32 = self.sprite.get_rotation();
        let velocity: Vector2 = Vector2::UP.rotated(rotation) * self.speed as f32;
        self.sprite.translate(velocity * delta as f32);

    }
}

