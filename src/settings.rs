use bevy::prelude::*;
use crate::core::{AppState, GameAssets, GameCameras};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Settings), setup_settings)
           .add_systems(Update, (
               settings_ui,
               handle_back_to_menu,
           ).run_if(in_state(AppState::Settings)))
           .add_systems(OnExit(AppState::Settings), cleanup_settings);
    }
}

fn setup_settings(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut game_cameras: ResMut<GameCameras>,
) {
    // Clean up existing camera
    if let Some(entity) = game_cameras.settings {
        commands.entity(entity).despawn();
    }

    // Spawn new camera
    let camera = commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 1,
            ..default()
        },
        ..default()
    }).id();
    game_cameras.settings = Some(camera);

    // Settings screen text
    commands.spawn(TextBundle {
        text: Text::from_section(
            "SETTINGS SCREEN",
            TextStyle {
                font: game_assets.font.clone(),
                font_size: 60.0,
                color: Color::WHITE,
            },
        ),
        style: Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.0),
            right: Val::Px(50.0),
            ..default()
        },
        ..default()
    });
}

fn settings_ui() {
    // Your settings UI logic will go here
}

fn handle_back_to_menu(
    mut next_state: ResMut<NextState<AppState>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Menu);
    }
}

fn cleanup_settings(
    mut commands: Commands,
    mut game_cameras: ResMut<GameCameras>,
) {
    if let Some(entity) = game_cameras.settings {
        commands.entity(entity).despawn();
        game_cameras.settings = None;
    }
}