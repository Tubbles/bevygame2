mod constant;
mod game;
// mod state;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    log::{Level, LogPlugin},
    prelude::*,
    render::settings::Backends,
    render::settings::WgpuSettings,
    render::RenderPlugin,
    window::close_on_esc,
    window::PresentMode,
};

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
                    filter: concat!(
                        "wgpu=warn,",
                        // "bevygame2=info,",
                        // "bevy=info,",
                    )
                    .to_string(),
                })
                .set(RenderPlugin {
                    wgpu_settings: WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..default()
                    },
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "I am a window!".into(),
                        // resolution: (640., 480.).into(),
                        present_mode: PresentMode::AutoVsync,
                        // // Tells wasm to resize the window according to the available canvas
                        // fit_canvas_to_parent: true,
                        // // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        // prevent_default_event_handling: false,
                        // window_theme: Some(WindowTheme::Dark),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_state::<AppState>()
        // .add_system(q_to_quit)
        .add_system(close_on_esc)
        .add_startup_system(spawn_camera)
        .insert_resource(ClearColor(Color::rgb(0.8, 0.8, 0.8)))
        .run();
}
