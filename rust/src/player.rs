use godot::{
    engine::{CharacterBody3D, CollisionShape3D, ICharacterBody3D, KinematicCollision3D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    speed: real,
    fall_acceleration: real,
    jump_impulse: real,
    bounce_impulse: real,
    target_velocity: Vector3,

    #[base]
    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Player {
            speed: 14.0,
            fall_acceleration: 75.0,
            jump_impulse: 20.0,
            bounce_impulse: 16.0,
            target_velocity: Vector3::ZERO,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let mut direction = Vector3::ZERO;
        let input = Input::singleton();

        if input.is_action_pressed("move_right".into()) {
            direction += Vector3::RIGHT;
        }
        if input.is_action_pressed("move_left".into()) {
            direction += Vector3::LEFT;
        }
        if input.is_action_pressed("move_forward".into()) {
            direction += Vector3::FORWARD;
        }
        if input.is_action_pressed("move_back".into()) {
            direction += Vector3::BACK;
        }

        if direction != Vector3::ZERO {
            let mut pivot = self.base().get_node_as::<Node3D>("Pivot");
            let position = self.base().get_position();
            let direction = direction.normalized();

            pivot.look_at(position + direction);
        }

        if self.base().is_on_floor() && input.is_action_just_pressed("jump".into()) {
            self.target_velocity.y = self.jump_impulse;
        }

        let index = self.base().get_slide_collision_count();
        for i in 0..index {
            let c = self
                .base()
                .get_node_as::<CollisionShape3D>("CollisionShape3D");

            let coll = self.base_mut().get_slide_collision(index).unwrap();
            let col = coll.get_collider().unwrap();
            let col = col.cast::<Node3D>();
            if col.is_in_group("mob".into()) {
                let mob = coll.get_collider().unwrap();

                if Vector3::UP.dot(coll.get_normal()) > 0.1 {
                    mob.squash();
                    self.target_velocity.y = self.bounce_impulse;
                    break;
                }
            }
        }

        let direction = direction.normalized();
        self.target_velocity.x = direction.x * self.speed;
        self.target_velocity.z = direction.z * self.speed;

        if !self.base().is_on_floor() {
            self.target_velocity.y -= self.fall_acceleration * delta as f32;
        }

        let v = self.target_velocity;
        self.base_mut().set_velocity(v);
        self.base_mut().move_and_slide();
    }
}

impl Player {}
