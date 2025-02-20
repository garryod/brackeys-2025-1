mod enemy;
mod player;

use self::{
    enemy::{EnemyBundle, EnemyMaterial, EnemyMesh, EnemyPlugin},
    player::{PlayerBundle, PlayerMaterial, PlayerMesh, PlayerPlugin},
};
use crate::{cleanup, fps_counter::FpsPlugin, AppState};
use bevy::{
    app::Plugin,
    math::Vec3,
    pbr::AmbientLight,
    prelude::{default, Camera3d, Commands, Component, OnEnter, OnExit, Res, Transform},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(FpsPlugin::default())
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_systems(OnEnter(AppState::Game), setup)
            .add_systems(OnExit(AppState::Game), cleanup::<Game>);
    }
}

#[derive(Component)]
struct Game;

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    player_mesh: Res<PlayerMesh>,
    player_material: Res<PlayerMaterial>,
    enemy_mesh: Res<EnemyMesh>,
    enemy_material: Res<EnemyMaterial>,
) {
    commands.spawn((
        Game,
        MainCamera,
        Camera3d::default(),
        Transform::from_xyz(-10., 5., 10.).looking_at(Vec3::ZERO, Vec3::Z),
    ));
    commands.spawn((
        Game,
        PlayerBundle::new(player_mesh, player_material, Vec3::new(0., 0., 0.)),
    ));
    commands.insert_resource(AmbientLight { ..default() });
    commands.spawn((
        Game,
        EnemyBundle::new(enemy_mesh, enemy_material, Vec3::new(2., 2., 0.)),
    ));
}
