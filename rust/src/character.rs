use godot::{prelude::*, engine::Area2D};

pub enum Status {
    Idle,
    Walk,
    Death,
    Attack,
}

// pub enum AreaType<'a> {
//     Hitbox(&'a str),
//     DetectionArea(&'a str),
// }

// pub struct Bodies {
//     hitbox: Option<Gd<Area2D>>,
//     detection_area: Option<Gd<Area2D>>,
// }
// pub trait DetectArea {
//     fn detect_area(&mut self, area_type: AreaType) -> Array<Gd<Node2D>>;
// }
trait Attack {
    fn attack_character(&self, character_health: i32);
}

