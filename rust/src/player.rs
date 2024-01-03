use crate::character::{Status};
use crate::enemy::Enemy;
use float_cmp::approx_eq;
use godot::engine::{AnimatedSprite2D, Area2D, CharacterBody2D, ICharacterBody2D, Timer};
use godot::obj::EngineClass;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    health: i32,
    speed: f64,
    prev_anim: Direction,

    #[base]
    base: Base<CharacterBody2D>,
}

#[derive(Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    Idle,
}

#[godot_api]
impl Player {
    // gets user-input, assigns a direction and then plays the appropriate animation for the key
    fn get_input(&mut self) -> Vector2 {
        let input: Gd<Input> = Input::singleton();

        let input_direction = input.get_vector(
            StringName::from("ui_left"),
            StringName::from("ui_right"),
            StringName::from("ui_up"),
            StringName::from("ui_down"),
        );
        let anim = self.get_animation_direction(input_direction);

        // If idle, we will animate using the *previous* direction
        // Else, update the previous animation state to be the current state and animate.
        match anim {
            Direction::Idle => {
                self.animate(self.prev_anim.clone(), Status::Idle);
            }
            _ => {
                self.prev_anim = anim.clone();
                self.animate(anim, Status::Walk);
            }
        }
        // godot_print!("{}", input_direction);
        input_direction
    }

    // performs floating-point comparisons to decide which direction to assign
    // since we don't have animations for every direction, if any side-to-side movement occurs we will look in that direction
    // when moving diagonally, the vector has components (0.707, 0.707) so we only need to check if the entries are almost 1.0
    fn get_animation_direction(&self, input: Vector2) -> Direction {
        let x: f32 = input.x;
        let y: f32 = input.y;

        if x > 0.0 {
            Direction::Right
        } else if x < 0.0 {
            Direction::Left
        } else if approx_eq!(f32, y, -1.0, ulps = 3) {
            Direction::Up
        } else if approx_eq!(f32, y, 1.0, ulps = 3) {
            // godot_print!("successfully recognised as down");
            Direction::Down
        } else {
            Direction::Idle
        }
    }

    // calls animation of the child animation node. don't know what happens if this doesn't exist (IT BETTER RAAAAH)
    fn animate(&self, current_dir: Direction, status: Status) {
        // get child animated sprite
        // if panics occuring then make sure the name is correct
        let mut anim = self
            .base
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        // string slice to append if moving
        let mut anim_name: &str = match status {
            Status::Idle => "_idle",
            Status::Walk => "_walk",
            _ => unimplemented!(),
        };

        match current_dir {
            Direction::Right => {
                anim.set_flip_h(false);
                anim.set_animation(StringName::from("side".to_owned() + anim_name));
            }
            Direction::Left => {
                anim.set_flip_h(true);
                anim.set_animation(StringName::from("side".to_owned() + anim_name));
            }
            Direction::Up => {
                anim.set_flip_h(false);
                anim.set_animation(StringName::from("back".to_owned() + anim_name));
            }
            Direction::Down => {
                anim.set_flip_h(false);
                anim.set_animation(StringName::from("front".to_owned() + anim_name));
            }
            Direction::Idle => {
                anim.set_flip_h(false);
                anim.set_animation(StringName::from("front_idle"));
                anim.play();
            }
        }
    }

    #[signal]
    fn hit();


    // TODO: Die at low health.
    #[func]
    fn on_hit(&mut self) {
        self.health -= 10;
        godot_print!("{}", self.health);

        if self.health <= 0 {
            godot_print!("Player has died :(");
            self.base.queue_free();
        }
    }

    #[func]
    fn on_enemy_enter_hitbox(&mut self, body: Gd<Node2D>) {
        let enemy = body;
        let mut timer = enemy.get_node_as::<Timer>("AttackCooldown");
        timer.connect("timeout".into(), enemy.callable("on_enemy_attack_reset"));

        // Here we re-enter the hitbox and if the cooldown has elapsed we apply on-hit and then start the timer.
        // This is needed over emitting the hit signal to prevent multiple mutable references to the same Gd<Player>.
        if timer.is_stopped() {
            self.on_hit();
            timer.start();
        }
    }

    #[func]
    fn on_enemy_exit_hitbox(&mut self, body: Gd<Node2D>) {
        let mut enemy = body;
        let mut timer = enemy.get_node_as::<Timer>("AttackCooldown");
        timer.disconnect("timeout".into(), enemy.callable("on_enemy_attack_reset"));
    }
}

#[godot_api]
pub impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            health: 100,
            speed: 100.0,
            prev_anim: Direction::Idle,
            base,
        }
    }

    // set velocity based on speed and user input to get direction
    fn physics_process(&mut self, _delta: f64) {
        let input = self.get_input();
        let velocity: Vector2 = input * self.speed as f32;

        self.base.set_velocity(velocity);
        self.base.move_and_slide();
    }
}
