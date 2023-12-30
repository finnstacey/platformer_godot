use float_cmp::approx_eq;
use godot::engine::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
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
                self.animate(self.prev_anim.clone(), false);
            },
            _ => {
                self.prev_anim = anim.clone();
                self.animate(anim, true);
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
    fn animate(&self, current_dir: Direction, moving: bool) {
        // get child animated sprite
        // if panics occuring then make sure the name is correct
        let mut anim = self.base.get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        // string slice to append if moving
        let mut status = "_walk";
        if moving {
            // pass
        } else {
            status = "_idle";
        }

        match current_dir {
            Direction::Right => {
                anim.set_flip_h(false);
                anim.set_animation(StringName::from("side".to_owned() + status));
            },
            Direction::Left => {
                anim.set_flip_h(true);
                anim.set_animation(StringName::from("side".to_owned() + status));
            },
            Direction::Up => {
                anim.set_flip_h(false);
                anim.set_animation(StringName::from("back".to_owned() + status));
            },
            Direction::Down => {
                anim.set_flip_h(false);
                anim.set_animation(StringName::from("front".to_owned() + status));
            },
            Direction::Idle => {
                anim.set_flip_h(false);
                anim.set_animation(StringName::from("front_idle"));
                anim.play();
            }
        }
    }
}

#[godot_api]
pub impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self { 
            speed: 100.0,
            prev_anim: Direction::Idle, 
            base }
    }

    // set velocity based on speed and user input to get direction
    fn physics_process(&mut self, _delta: f64) {
        let input = self.get_input();
        let velocity: Vector2 = input * self.speed as f32;

        self.base.set_velocity(velocity);
        self.base.move_and_slide();
    }
}
