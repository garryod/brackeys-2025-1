mod enemy;
mod player;

use self::player::{PlayerBundle, PlayerPlugin};
use crate::{cleanup, fps_counter::FpsPlugin, AppState};
use bevy::{
    app::Plugin,
    asset::Assets,
    math::Vec3,
    pbr::{AmbientLight, StandardMaterial},
    prelude::{default, Camera3d, Commands, Component, Mesh, OnEnter, OnExit, ResMut, Transform},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(FpsPlugin::default())
            .add_plugins(PlayerPlugin)
            .add_systems(OnEnter(AppState::Game), setup)
            .add_systems(OnExit(AppState::Game), cleanup::<Game>);
    }
}

#[derive(Component)]
struct Game;

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Game,
        Camera3d::default(),
        Transform::from_xyz(-10., 5., 10.).looking_at(Vec3::ZERO, Vec3::Z),
    ));
    commands.spawn((
        Game,
        PlayerBundle::new(meshes, materials),
        Transform::from_xyz(0., 1., 0.),
    ));
    commands.insert_resource(AmbientLight { ..default() });
}
