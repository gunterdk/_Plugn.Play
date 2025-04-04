use bevy::prelude::*;
use crate::core::{AppState, GameAssets, GameCameras};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_game)
           .add_systems(Update, (
               game_controls,
               handle_back_to_menu,
           ).run_if(in_state(AppState::InGame)))
           .add_systems(OnExit(AppState::InGame), cleanup_game);
    }
}

fn setup_game(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut game_cameras: ResMut<GameCameras>,
) {
    // Clean up existing camera
    if let Some(entity) = game_cameras.game {
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
    game_cameras.game = Some(camera);

    // Game screen text
    commands.spawn(TextBundle {
        text: Text::from_section(
            "GAME SCREEN",
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

fn game_controls() {
    // Your game logic will go here
}

fn handle_back_to_menu(
    mut next_state: ResMut<NextState<AppState>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Menu);
    }
}

fn cleanup_game(
    mut commands: Commands,
    mut game_cameras: ResMut<GameCameras>,
) {
    if let Some(entity) = game_cameras.game {
        commands.entity(entity).despawn();
        game_cameras.game = None;
    }
}