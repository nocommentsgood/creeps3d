use std::ops::Neg;

use godot::{
    engine::{CharacterBody3D, ICharacterBody3D, VisibleOnScreenNotifier3D},
    prelude::*,
};
use rand::Rng;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Enemy {
    min_speed: real,
    max_speed: real,

    #[base]
    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Enemy {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Enemy {
            min_speed: 10.0,
            max_speed: 18.0,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl Enemy {
    #[func]
    pub fn initialize(&mut self, start_position: Vector3, player_position: Vector3) {
        let mut rng = rand::thread_rng();
        let p_pi = std::f32::consts::PI / 4.0;
        let n_pi = std::f32::consts::PI.neg() / 4.0;

        self.base_mut()
            .look_at_from_position(start_position, player_position);

        self.base_mut().rotate_y(rng.gen_range(n_pi..p_pi));

        let rand_speed = rng.gen_range(self.min_speed..self.max_speed);
        let velocity = Vector3::FORWARD * rand_speed;
        let rotation = self.base().get_rotation();
        self.base_mut()
            .set_velocity(velocity.rotated(Vector3::UP, rotation.y));
    }

    #[func]
    fn on_screen_exited(&mut self) {
        let mut notifier = self
            .base_mut()
            .get_node_as::<VisibleOnScreenNotifier3D>("VisibleOnScreenNotifier3D");
        notifier.queue_free();
    }
}
