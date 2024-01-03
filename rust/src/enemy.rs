use crate::character::Status;
use godot::engine::{CharacterBody2D, ICharacterBody2D, AnimatedSprite2D, Timer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Enemy {
    health: i32,
    speed: f64,
    player: Gd<CharacterBody2D>,

    #[base]
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl Enemy {
    #[signal]
    fn hit();

    #[func]
    fn on_hit(&mut self) {
        self.health -= 10;
        godot_print!("{}", self.health);
        godot_print!("Successful attack");

        if self.health <= 0 {
            godot_print!("Enemy has died :(");
            self.base.queue_free();
        }
    }

    #[func]
    fn on_player_entered_detection_area(&mut self, body: Gd<CharacterBody2D>) {
        self.player = body;
        self.base.set_physics_process(true);
        self.animate(Status::Walk);

    }

    #[func]
    fn on_player_exited_detection_area(&mut self, _body: Gd<CharacterBody2D>) {
        self.base.set_physics_process(false);
        self.animate(Status::Idle);

    }

    #[func]
    fn on_enemy_attack_reset(&mut self) {
        self.player.emit_signal("hit".into(), &[]);
        let mut timer = self.base.get_node_as::<Timer>("AttackCooldown");
        timer.start();
    }

    fn animate(&self, status: Status) {
        let mut anim = self.base.get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        match status {
            Status::Idle => anim.set_animation(StringName::from("idle")),
            Status::Walk => anim.set_animation(StringName::from("walk")),
            _ => todo!()
        }

        anim.play();
    }
}

#[godot_api]
pub impl ICharacterBody2D for Enemy {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            health: 100,
            speed: 50.0,
            player: CharacterBody2D::new_alloc(),

            base,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        // let timer = self.base.get_node_as::<Timer>("AttackCooldown");
        // let time_remaining: f64 = timer.get_time_left();
        // godot_print!("{}", time_remaining);
        let player_pos: Vector2 = self.player.get_position();
        let chase_direction: Vector2 = (player_pos - self.base.get_position()).normalized(); // all directions should be unit vectors
        let velocity: Vector2 = chase_direction * self.speed as f32;

        self.base.set_velocity(velocity);
        self.base.move_and_slide();
    }
}
