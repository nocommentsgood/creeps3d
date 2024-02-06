use godot::{
    engine::{ILabel, Label},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Label)]
pub struct Score {
    score: i64,
    base: Base<Label>,
}

#[godot_api]
impl ILabel for Score {
    fn init(base: Base<Label>) -> Self {
        Score { score: 0, base }
    }
}

#[godot_api]
impl Score {
    #[func]
    fn on_mob_squashed(&mut self) {
        // TODO: This is not working
        self.score += 1;
        let score = format!("Score is: {}", self.score);
        self.base_mut().set_text(score.into());
    }
}
