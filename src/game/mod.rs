mod player;

use bevy::prelude::*;

use player::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            // .add_event::<GameOver>()
            // States
            // .add_state::<SimulationState>()
            // OnEnter Systems
            // .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            // My Plugins
            // .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            // .add_plugin(ScorePlugin)
            // .add_plugin(StarPlugin)
            // .add_plugin(GameUIPlugin)
            // Systems
            // .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            // Exit State Systems
            // .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)))
            ;
    }
}
