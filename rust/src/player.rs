use godot::{
    engine::{AnimationPlayer, CharacterBody3D, ICharacterBody3D},
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

        // prevent moving faster when moving diagonally
        if direction != Vector3::ZERO {
            let mut pivot = self.base().get_node_as::<Node3D>("Pivot");
            let position = self.base().get_position();
            let direction = direction.normalized();
            let mut animation = self
                .base_mut()
                .get_node_as::<AnimationPlayer>("AnimationPlayer");

            pivot.look_at(position + direction);
            animation.set_speed_scale(4.0);
        } else {
            let mut animation = self
                .base_mut()
                .get_node_as::<AnimationPlayer>("AnimationPlayer");
            animation.set_speed_scale(1.0);
        }

        // moving
        if self.base().is_on_floor() && input.is_action_just_pressed("jump".into()) {
            self.target_velocity.y = self.jump_impulse;
        }

        // vertical moving
        let direction = direction.normalized();
        self.target_velocity.x = direction.x * self.speed;
        self.target_velocity.z = direction.z * self.speed;

        // jumping
        if !self.base().is_on_floor() {
            self.target_velocity.y -= self.fall_acceleration * delta as f32;
        }

        // squashing
        let count = self.base().get_slide_collision_count();
        for i in 0..count {
            let collision = self.base_mut().get_slide_collision(i);

            // gross
            // TODO: Refactor this
            if let Some(kin_3d) = collision {
                let collider = kin_3d.get_collider();
                if let Some(col) = collider {
                    let col: Gd<Node> = col.cast();
                    if col.is_in_group("mob".into()) {
                        let mut mob: Gd<crate::enemy::Enemy> = col.cast();
                        if Vector3::UP.dot(kin_3d.get_normal()) > 0.1 {
                            mob.bind_mut().squash();
                            self.target_velocity.y = self.bounce_impulse;
                            break;
                        }
                    }
                }
            }
        }

        let v = self.target_velocity;
        self.base_mut().set_velocity(v);
        self.base_mut().move_and_slide();

        let mut pivot = self.base_mut().get_node_as::<Node3D>("Pivot");
        // pivot.rotate_x(std::f32::consts::PI / 6.0 * self.target_velocity.y / self.jump_impulse);
        let mut pivot_roation = pivot.get_rotation();
        pivot_roation.x = std::f32::consts::PI / 6.0 * self.target_velocity.y / self.jump_impulse;
        pivot.set_rotation(pivot_roation);
    }
}

#[godot_api]
impl Player {
    #[signal]
    fn hit();

    #[func]
    fn die(&mut self) {
        self.base_mut().emit_signal("hit".into(), &[]);
        self.base_mut().queue_free();
    }

    #[func]
    fn on_mob_detector_entered(&mut self) {
        self.die();
    }
}
