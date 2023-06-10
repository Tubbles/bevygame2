use bevy::prelude::*;

#[derive(Resource)]
pub struct Constant {
    pub aasd: usize,
}

impl Default for Constant {
    fn default() -> Constant {
        Constant { aasd: 42 }
    }
}
