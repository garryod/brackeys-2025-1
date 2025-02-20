mod enemy;
mod objective;
mod player;

use self::{
    enemy::{EnemyBundle, EnemyMaterial, EnemyMesh, EnemyPlugin},
    objective::{ObjectiveBundle, ObjectiveMaterial, ObjectiveMesh, ObjectivePlugin},
    player::{PlayerBundle, PlayerMaterial, PlayerMesh, PlayerPlugin},
};
use crate::{cleanup, fps_counter::FpsPlugin, AppState};
use bevy::prelude::{
    default, AmbientLight, Camera3d, Commands, Component, OnEnter, OnExit, Plugin, Res, Transform,
    Vec2, Vec3,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(FpsPlugin::default())
            .add_plugins(PlayerPlugin)
            .add_plugins(ObjectivePlugin)
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
    objective_mesh: Res<ObjectiveMesh>,
    objective_material: Res<ObjectiveMaterial>,
    enemy_mesh: Res<EnemyMesh>,
    enemy_material: Res<EnemyMaterial>,
) {
    commands.spawn((
        Game,
        MainCamera,
        Camera3d::default(),
        Transform::from_xyz(-10., 5., 10.).looking_at(Vec3::ZERO, Vec3::Z),
    ));
    commands.spawn(ObjectiveBundle::new(
        objective_mesh,
        objective_material,
        Vec2::new(10., 0.),
    ));
    commands.spawn((
        Game,
        PlayerBundle::new(player_mesh, player_material, Vec2::new(0., 0.)),
    ));
    commands.insert_resource(AmbientLight { ..default() });
    commands.spawn((
        Game,
        EnemyBundle::new(enemy_mesh, enemy_material, Vec2::new(2., 2.)),
    ));
}
