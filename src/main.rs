use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy::winit::{WinitSettings, UpdateMode};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::utils::Duration;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(LogDiagnosticsPlugin::default())
    .insert_resource(
        WinitSettings {
            focused_mode: UpdateMode::Reactive {max_wait: Duration::MAX},
            unfocused_mode: UpdateMode::Reactive {max_wait: Duration::MAX},
            return_from_run: false,
        }
    )
    .add_startup_system(setup)
    .add_system(close_on_esc)
    .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                ..default()
            },
            background_color: Color::RED.into(),
            ..default()
        }
    )
    .with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "Hello world!",
                TextStyle {
                    font: assets.load("FSEX300.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                }
            ),
            Label,
        ));
    });
    commands.spawn(
        NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                ..default()
            },
            background_color: Color::RED.into(),
            ..default()
        }
    )
    .with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "Hello world!",
                TextStyle {
                    font: assets.load("FSEX300.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                }
            ),
            Label,
        ));
    });
}