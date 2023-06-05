use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Configure System Sets
            .add_startup_system(hello_world)
            // .configure_set(MovementSystemSet.before(ConfinementSystemSet))
            // On Enter State
            // .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            // Systems
            // .add_systems(
            //     (
            //         player_movement.in_set(MovementSystemSet),
            //         confine_player_movement.in_set(ConfinementSystemSet),
            //     )
            //         .in_set(OnUpdate(AppState::Game))
            //         .in_set(OnUpdate(SimulationState::Running)),
            // )
            // .add_systems(
            //     (enemy_hit_player, player_hit_star)
            //         .in_set(OnUpdate(AppState::Game))
            //         .in_set(OnUpdate(SimulationState::Running)),
            // )
            // On Exit State
            // .add_system(despawn_player.in_schedule(OnExit(AppState::Game)))
            ;
    }
}

fn hello_world() {
    println!("Hello, world!");
}