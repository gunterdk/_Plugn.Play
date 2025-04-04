use bevy::prelude::*;

// Game states
#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
    Settings,
}

// UI assets
#[derive(Resource, Default)]
pub struct GameAssets {
    pub font: Handle<Font>,
}

// Button types
#[derive(Component)]
pub struct MenuButton {
    pub button_type: ButtonType,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonType {
    Start,
    Settings,
    Quit,
}

// Camera tracking
#[derive(Resource, Default)]
pub struct GameCameras {
    pub menu: Option<Entity>,
    pub game: Option<Entity>,
    pub settings: Option<Entity>,
}