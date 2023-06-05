use bevy::prelude::*;

use bevy::app::AppExit;

mod game;
mod state;

use game::*;
use state::*;

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        // My Plugins
        // .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        // Startup Systems
        // .add_startup_system(spawn_camera)
        // Systems
        // .add_system(transition_to_game_state)
        // .add_system(transition_to_main_menu_state)
        .add_system(exit_game)
        // .add_system(handle_game_over)
        .run();
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Q) {
        app_exit_event_writer.send(AppExit);
    }
}
