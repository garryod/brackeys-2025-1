use bevy::{
    asset::Handle,
    image::Image,
    prelude::{
        AlphaMode, App, Assets, Bundle, Camera, Component, Mesh, Mesh3d, MeshMaterial3d, Plane3d,
        Plugin, Query, ResMut, StandardMaterial, Transform, Vec2, Vec3, With,
    },
    render::render_resource::Face,
    utils::default,
};

pub struct Sprite3DPlugin;

impl Plugin for Sprite3DPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component, Default)]
#[require(Mesh3d, MeshMaterial3d<StandardMaterial>)]
struct Sprite3D;

#[derive(Component, Default)]
#[require(Sprite3D)]
struct Billboard;

#[derive(Bundle)]
pub struct Sprite3DBundle {
    marker: Sprite3D,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
}

impl Sprite3DBundle {
    pub fn new(
        size: Vec2,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        image: Handle<Image>,
    ) -> Self {
        Self {
            marker: Sprite3D,
            mesh: Mesh3d(meshes.add(Plane3d::new(Vec3::Z, size / 2.))),
            material: MeshMaterial3d(materials.add(StandardMaterial {
                alpha_mode: AlphaMode::Mask(0.5),
                base_color_texture: Some(image),
                cull_mode: Some(Face::Back),
                perceptual_roughness: 0.5,
                reflectance: 0.15,
                ..default()
            })),
        }
    }
}

fn bilboard(
    sprites: Query<&mut Transform, With<Billboard>>,
    camera: Query<Transform, With<Camera>>,
) {
}
