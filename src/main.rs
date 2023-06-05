mod game;
mod state;

use bevy::prelude::*;
// use bevy::window::close_on_esc;

use bevy::app::AppExit;
use bevy::app::CoreSchedule::Startup;
use bevy::window::PrimaryWindow;

use game::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

fn main() {
    App::new()
        .add_plugin(GamePlugin)
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_system(q_to_quit)
        .add_system(spawn_camera.in_schedule(Startup))
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
pub fn q_to_quit(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Q) {
        app_exit_event_writer.send(AppExit);
    }
}
