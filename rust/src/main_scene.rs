use crate::enemy::Enemy;
use godot::{
    engine::{CharacterBody3D, PathFollow3D, Timer},
    prelude::*,
};
use rand::Rng;

#[derive(GodotClass)]
#[class(base=Node)]
struct Main {
    mob_scene: Gd<PackedScene>,

    base: Base<Node>,
}

#[godot_api]
impl Main {
    #[func]
    fn on_mob_timer_timeout(&mut self) {
        let mut mob_spawn_loc = self
            .base()
            .get_node_as::<PathFollow3D>("SpawnLocation/SpawnPath");
        let mob_scene = self.mob_scene.instantiate_as::<CharacterBody3D>();
        let mut rng = rand::thread_rng();
        let player_position = self.base().get_node_as::<CharacterBody3D>("Player");
        let mut mob_scene = mob_scene.cast::<Enemy>();

        mob_spawn_loc.set_progress_ratio(rng.gen::<f32>());

        {
            let mut mob = mob_scene.bind_mut();
            mob.initialize(mob_spawn_loc.get_position(), player_position.get_position());
        }

        self.base_mut().add_child(mob_scene.clone().upcast());
    }

    #[func]
    fn on_player_hit(&mut self) {
        let mut timer = self.base().get_node_as::<Timer>("MobTimer");
        timer.stop();
    }
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        Main {
            mob_scene: PackedScene::new_gd(),
            base,
        }
    }

    fn ready(&mut self) {
        self.mob_scene = load("res://mob.tscn");
    }
}
