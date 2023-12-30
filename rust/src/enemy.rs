use godot::engine::{Area2D, CharacterBody2D, ICharacterBody2D, AnimatedSprite2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Enemy {
    speed: f64,
    detection_area: Option<Gd<Area2D>>,

    #[base]
    base: Base<CharacterBody2D>,
}

pub enum Status {
    Idle,
    Walk,
    Death,
    Attack,
}

impl Enemy {
    fn detect_bodies(&mut self) -> Array<Gd<Node2D>> {
        // Object will be initialised as None. On first detection we need to obtain child node.
        if let None = self.detection_area {
            self.detection_area = Some(self.base.get_node_as::<Area2D>("DetectionArea"));
        }

        // the below statement shouldn't panic if you have the nodes setup correctly.
        let detected_bodies = self.detection_area.as_ref().expect("Enemy must have child node DetectionArea of type Area2D for detect_bodies() to run.").get_overlapping_bodies();
        // let detected_bodies = self.detection_area.as_ref().unwrap().get_overlapping_bodies();
        detected_bodies
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
            speed: 50.0,
            detection_area: None,

            base,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        // Detect bodies
        let detected_bodies: Array<Gd<Node2D>> = self.detect_bodies();
        
        // Chase the detected body. For now, there is only one player so we should access the first entry.
        // TODO: Upon touching the player he should pause.
        if detected_bodies.len() > 0 {
            let player_pos: Vector2 = detected_bodies.get(0).get_position(); // this should not panic :D
            let chase_direction: Vector2 = (player_pos - self.base.get_position()).normalized(); // all directions should be unit vectors
            let velocity: Vector2 = chase_direction * self.speed as f32;

            self.base.set_velocity(velocity);
            self.base.move_and_slide();
            self.animate(Status::Walk);
        } else {
            self.animate(Status::Idle);
        }
    }
}
