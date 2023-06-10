mod constant;
mod game;
// mod state;

use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::render::settings::Backends;
use bevy::render::settings::WgpuSettings;
use bevy::render::RenderPlugin;
use bevy::window::close_on_esc;
use constant::Constant;
use game::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

// fn q_to_quit(keyboard_input: Res<Input<KeyCode>>, mut app_exit_event_writer: EventWriter<AppExit>) {
//     if keyboard_input.just_pressed(KeyCode::Q) {
//         app_exit_event_writer.send(AppExit);
//     }
// }

fn main() {
    App::new()
        .insert_resource(Constant::default())
        .add_plugin(GamePlugin)
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::INFO,
                    // filter: "wgpu=error,bevy_render=info,bevy_ecs=trace".to_string(),
                    filter: concat!(
                        "bevy_diagnostic=warn,",
                        "bevy_render=warn,",
                        "bevy_window=warn,",
                        "bevy_winit=warn,",
                        "bevy_winit=warn,",
                        "wgpu_core=warn,",
                        "wgpu_hal=warn,",
                        "winit=warn,",
                    )
                    .to_string(),
                })
                .set(RenderPlugin {
                    wgpu_settings: WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..default()
                    },
                }),
        )
        .add_state::<AppState>()
        // .add_system(q_to_quit)
        .add_system(close_on_esc)
        .add_startup_system(spawn_camera)
        .insert_resource(ClearColor(Color::rgb(0.8, 0.8, 0.8)))
        .run();
}
