mod enemy;
mod main_scene;
mod player;

use godot::prelude::*;

struct Template;

#[gdextension]
unsafe impl ExtensionLibrary for Template {}
