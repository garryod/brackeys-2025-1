use super::player::Player;
use crate::AppState;
use bevy::prelude::{
    default, in_state, Added, App, AssetServer, Assets, BuildChildren, Bundle, Camera, Capsule3d,
    ChildBuild, Color, Commands, Component, Entity, GlobalTransform, Handle, Image,
    IntoSystemConfigs, Mesh, Mesh3d, MeshMaterial3d, Parent, Plugin, Quat, Query,
    RemovedComponents, Res, ResMut, Resource, StandardMaterial, Startup, Transform, Update, Vec2,
    Vec3Swizzles, With,
};
use bevy_sprite3d::{Sprite3dBuilder, Sprite3dParams, Sprite3dPlugin};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Sprite3dPlugin)
            .insert_resource(AlertSprite::default())
            .insert_resource(EnemyMesh::default())
            .insert_resource(EnemyMaterial::default())
            .add_systems(Startup, load)
            .add_systems(
                Update,
                (add_alert_icon, remove_alert_icon, face_alert_icon, alert)
                    .run_if(in_state(AppState::Game)),
            );
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
}

impl EnemyBundle {
    pub fn new(
        enemy_mesh: Res<EnemyMesh>,
        enemy_material: Res<EnemyMaterial>,
        position: Vec2,
    ) -> Self {
        Self {
            enemy: Enemy,
            mesh: Mesh3d(enemy_mesh.0.clone()),
            material: MeshMaterial3d(enemy_material.0.clone()),
            transform: Transform::from_xyz(position.x, position.y, 0.)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        }
    }
}

#[derive(Component)]
struct Enemy;

#[derive(Component, Default)]
struct Alerted;

#[derive(Component)]
struct AlertIcon;

#[derive(Resource, Default)]
pub struct EnemyMesh(Handle<Mesh>);

#[derive(Resource, Default)]
pub struct EnemyMaterial(Handle<StandardMaterial>);

#[derive(Resource, Default)]
struct AlertSprite(Handle<Image>);

fn load(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut enemy_mesh: ResMut<EnemyMesh>,
    mut enemy_material: ResMut<EnemyMaterial>,
    mut alert_sprite: ResMut<AlertSprite>,
) {
    enemy_mesh.0 = meshes.add(Capsule3d::new(0.25, 2.));
    enemy_material.0 = materials.add(Color::hsl(0., 0.6, 0.5));
    alert_sprite.0 = asset_server.load("sprites/enemy/alert.png")
}

fn add_alert_icon(
    alerted: Query<Entity, Added<Alerted>>,
    mut commands: Commands,
    mut sprite_params: Sprite3dParams,
    sprite: Res<AlertSprite>,
) {
    for enemy in &alerted {
        commands.entity(enemy).with_children(|enemy| {
            enemy.spawn((
                AlertIcon,
                Sprite3dBuilder {
                    image: sprite.0.clone(),
                    pixels_per_metre: 16.,
                    ..default()
                }
                .bundle(&mut sprite_params),
                Transform::from_xyz(0., 2.5, 0.),
            ));
        });
    }
}

fn remove_alert_icon(
    mut unalerted: RemovedComponents<Alerted>,
    icons: Query<(Entity, &Parent), With<AlertIcon>>,
    mut commands: Commands,
) {
    let unalerted = unalerted.read().collect::<Vec<_>>();
    for (entity, parent) in &icons {
        if unalerted.contains(parent) {
            commands.entity(entity).despawn();
        }
    }
}

fn face_alert_icon(
    mut transforms: Query<(&mut Transform, &GlobalTransform), With<AlertIcon>>,
    camera_position: Query<&GlobalTransform, With<Camera>>,
) {
    let camera_position = camera_position.single().translation().xy();
    for (mut local, global) in transforms.iter_mut() {
        let angle =
            (global.translation().xy() - camera_position).to_angle() + std::f32::consts::FRAC_PI_2;
        local.rotation = Quat::from_rotation_y(angle);
    }
}

fn alert(
    enemies: Query<(Entity, &GlobalTransform, Option<&Alerted>), With<Enemy>>,
    player: Query<&GlobalTransform, With<Player>>,
    mut commands: Commands,
) {
    let player_transform = player.single();
    for (entity, transform, alerted) in &enemies {
        let separation =
            (transform.translation().xy() - player_transform.translation().xy()).length();
        match alerted.is_some() {
            true if separation >= 2. => {
                commands.entity(entity).remove::<Alerted>();
            }
            false if separation < 1. => {
                commands.entity(entity).insert(Alerted);
            }
            _ => {}
        }
    }
}
