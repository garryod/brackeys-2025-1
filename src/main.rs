mod fps_counter;
mod game;
mod menu;

use crate::{game::GamePlugin, menu::MenuPlugin};
use bevy::{
    app::App,
    prelude::{
        AppExtStates, Commands, Component, DespawnRecursiveExt, Entity, Query, States, With,
    },
    DefaultPlugins,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
        .init_state::<AppState>()
        .run();
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    Game,
}

pub fn cleanup<T: Component>(query: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
