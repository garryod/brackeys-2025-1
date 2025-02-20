use bevy::{
    app::Startup,
    asset::Handle,
    color::Color,
    prelude::{
        Assets, Bundle, Component, Cuboid, Mesh, Mesh3d, MeshMaterial3d, Plugin, Res, ResMut,
        Resource, StandardMaterial, Transform, Vec2,
    },
};

pub struct ObjectivePlugin;

impl Plugin for ObjectivePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ObjectiveMesh::default())
            .insert_resource(ObjectiveMaterial::default())
            .add_systems(Startup, load);
    }
}

#[derive(Bundle)]
pub struct ObjectiveBundle {
    marker: Objective,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
}

impl ObjectiveBundle {
    pub fn new(
        objective_mesh: Res<ObjectiveMesh>,
        objective_material: Res<ObjectiveMaterial>,
        position: Vec2,
    ) -> Self {
        Self {
            marker: Objective,
            mesh: Mesh3d(objective_mesh.0.clone()),
            material: MeshMaterial3d(objective_material.0.clone()),
            transform: Transform::from_xyz(position.x, position.y, 0.),
        }
    }
}

#[derive(Component)]
struct Objective;

#[derive(Resource, Default)]
pub struct ObjectiveMesh(Handle<Mesh>);

#[derive(Resource, Default)]
pub struct ObjectiveMaterial(Handle<StandardMaterial>);

fn load(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut objective_mesh: ResMut<ObjectiveMesh>,
    mut objective_material: ResMut<ObjectiveMaterial>,
) {
    objective_mesh.0 = meshes.add(Cuboid::new(0.5, 0.5, 0.2));
    objective_material.0 = materials.add(Color::hsl(140., 0.2, 0.5));
}
