use crate::AppState;

use super::player::Player;
use bevy::prelude::{
    in_state, App, Assets, Bundle, Capsule3d, Color, Component, Deref, DerefMut, IntoSystemConfigs,
    Mesh, Mesh3d, MeshMaterial3d, Plugin, Quat, Query, ResMut, StandardMaterial, Transform, Update,
    Vec3, With,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, alert.run_if(in_state(AppState::Game)));
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    alerted: Alerted,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
}

impl EnemyBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        position: Vec3,
    ) -> Self {
        Self {
            enemy: Enemy,
            alerted: Alerted(false),
            mesh: Mesh3d(meshes.add(Capsule3d::new(0.25, 2.))),
            material: MeshMaterial3d(materials.add(Color::hsl(0., 0.6, 0.5))),
            transform: Transform::from_translation(position)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        }
    }
}

#[derive(Component)]
struct Enemy;

#[derive(Component, Default, Deref, DerefMut)]
struct Alerted(bool);

fn alert(mut enemies: Query<(&Transform, &mut Alerted)>, player: Query<&Transform, With<Player>>) {
    let player_transform = player.single();
    for (enemy_transform, mut alerted) in enemies.iter_mut() {
        let separation = (enemy_transform.translation - player_transform.translation).length();
        match **alerted {
            true if separation >= 20. => **alerted = false,
            false if separation < 10. => **alerted = true,
            _ => {}
        }
    }
}
