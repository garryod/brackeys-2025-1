use crate::{cleanup, fps_counter::FpsPlugin, AppState};
use bevy::{
    app::Plugin,
    prelude::{Camera3d, Commands, Component, OnEnter, OnExit},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(FpsPlugin::default())
            .add_systems(OnEnter(AppState::Game), setup)
            .add_systems(OnExit(AppState::Game), cleanup::<Game>);
    }
}

#[derive(Component)]
struct Game;

fn setup(mut commands: Commands) {
    commands.spawn((Game, Camera3d::default()));
}
