use bevy::prelude::*;
use bevy::app::AppExit;
mod core;
mod game;
mod settings;

use core::{AppState, GameAssets, ButtonType, MenuButton, GameCameras};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Plug & Play".into(),
                    resolution: (1280.0, 720.0).into(),
                    ..default()
                }),
                close_when_requested: true,
                exit_condition: bevy::window::ExitCondition::OnAllClosed,
                ..default()
            }),
            game::GamePlugin,
            settings::SettingsPlugin,
        ))
        .init_resource::<GameAssets>()
        .init_resource::<GameCameras>()
        .add_state::<AppState>()
        .add_systems(Startup, setup_menu)
        .add_systems(Update, (
            button_system,
            handle_state_transitions,
        ).run_if(in_state(AppState::Menu)))
        .run();
}

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_assets: ResMut<GameAssets>,
    mut game_cameras: ResMut<GameCameras>,
) {
    // Clean up existing camera
    if let Some(entity) = game_cameras.menu {
        commands.entity(entity).despawn();
    }

    // Load font
    game_assets.font = asset_server.load("fonts/Daydream.ttf");

    // Spawn new camera
    let camera = commands.spawn(Camera2dBundle::default()).id();
    game_cameras.menu = Some(camera);
    
    // UI root node
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: Color::rgba(0.1, 0.1, 0.15, 0.9).into(),
        ..default()
    }).with_children(|parent| {
        // Title
        parent.spawn(TextBundle::from_section(
            "PLUG & PLAY",
            TextStyle {
                font: game_assets.font.clone(),
                font_size: 96.0,
                color: Color::WHITE,
            },
        ));

        // Spacer
        parent.spawn(NodeBundle {
            style: Style {
                height: Val::Px(60.0),
                ..default()
            },
            ..default()
        });

        // Buttons
        spawn_button(parent, "START", ButtonType::Start, &game_assets);
        spawn_button(parent, "SETTINGS", ButtonType::Settings, &game_assets);
        spawn_button(parent, "QUIT", ButtonType::Quit, &game_assets);
    });
}

fn spawn_button(parent: &mut ChildBuilder, text: &str, button_type: ButtonType, assets: &GameAssets) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(250.0),
                height: Val::Px(80.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            },
            background_color: Color::rgba(0.2, 0.2, 0.3, 0.8).into(),
            ..default()
        },
        MenuButton { button_type },
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font: assets.font.clone(),
                font_size: 36.0,
                color: Color::WHITE,
            },
        ));
    });
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgba(0.0, 0.7, 0.0, 1.0).into();
                if let ButtonType::Quit = button.button_type {
                    app_exit.send(AppExit);
                }
            }
            Interaction::Hovered => {
                *color = Color::rgba(0.3, 0.3, 0.4, 0.9).into();
            }
            Interaction::None => {
                *color = Color::rgba(0.2, 0.2, 0.3, 0.8).into();
            }
        }
    }
}

fn handle_state_transitions(
    mut next_state: ResMut<NextState<AppState>>,
    interaction_query: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button.button_type {
                ButtonType::Start => next_state.set(AppState::InGame),
                ButtonType::Settings => next_state.set(AppState::Settings),
                ButtonType::Quit => {},
            }
        }
    }
}