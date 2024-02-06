mod enemy;
mod main_scene;
mod player;
mod score;

use godot::prelude::*;

struct Template;

#[gdextension]
unsafe impl ExtensionLibrary for Template {}
