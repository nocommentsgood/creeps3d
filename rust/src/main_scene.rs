use godot::{
    engine::{CharacterBody3D, ColorRect, InputEvent, Label, PathFollow3D, Timer},
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
        let mut mob_scene = self.mob_scene.instantiate_as::<crate::enemy::Enemy>();
        let mut rng = rand::thread_rng();
        let player_position = self.base().get_node_as::<CharacterBody3D>("Player");

        mob_spawn_loc.set_progress_ratio(rng.gen::<f32>());

        {
            let mut mob = mob_scene.bind_mut();
            mob.initialize(mob_spawn_loc.get_position(), player_position.get_position());
        }

        self.base_mut().add_child(mob_scene.clone().upcast());

        // this is causing:
        // signal is already connected to callable
        // error
        let mut mob = self.base_mut().get_node_as::<CharacterBody3D>("Mob");
        let label = self
            .base_mut()
            .get_node_as::<Label>("UserInterface/ScoreLabel");
        {
            mob.connect("squashed".into(), label.callable("on_mob_squashed"));
        }
    }

    #[func]
    fn on_player_hit(&mut self) {
        let mut timer = self.base().get_node_as::<Timer>("MobTimer");
        let mut retry = self.base().get_node_as::<ColorRect>("UserInterface/Retry");
        timer.stop();
        retry.show();
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

        let mut retry = self.base().get_node_as::<ColorRect>("UserInterface/Retry");
        retry.hide();
    }

    fn unhandled_input(&mut self, input: Gd<InputEvent>) {
        let retry = self.base().get_node_as::<ColorRect>("UserInterface/Retry");
        if input.is_action_pressed("ui_accept".into()) && retry.is_visible() {
            self.base_mut().get_tree().unwrap().reload_current_scene();
        }
    }
}
