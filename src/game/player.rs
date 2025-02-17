use super::MainCamera;
use crate::AppState;
use bevy::{
    prelude::{
        in_state, App, Assets, Bundle, Camera, Color, Component, Cuboid, Dir2, GamepadButton,
        GlobalTransform, InfinitePlane3d, IntoSystemConfigs, KeyCode, Mesh, Mesh3d, MeshMaterial3d,
        Plugin, Quat, Query, Reflect, Res, ResMut, Resource, StandardMaterial, Time, Transform,
        Update, Vec2, Vec3, Vec3Swizzles, Window, With,
    },
    window::PrimaryWindow,
};
use leafwing_input_manager::{
    plugin::InputManagerPlugin,
    prelude::{ActionState, InputMap},
    Actionlike,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .init_resource::<CursorCoordinates>()
            .add_systems(
                Update,
                (project_cursor, move_player).run_if(in_state(AppState::Game)),
            );
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    action_state: ActionState<Action>,
    input_map: InputMap<Action>,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
}

impl PlayerBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        position: Vec3,
    ) -> Self {
        Self {
            player: Player,
            action_state: ActionState::<Action>::default(),
            input_map: Action::default_input_map(),
            mesh: Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 2.))),
            material: MeshMaterial3d(materials.add(Color::hsl(220., 0.1, 0.5))),
            transform: Transform::from_translation(position),
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Debug, Actionlike, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
enum Action {
    Up,
    Down,
    Left,
    Right,
}

impl Action {
    const DIRECTIONS: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];

    fn direction(&self) -> Option<Dir2> {
        match self {
            Self::Up => Some(Dir2::X),
            Self::Down => Some(Dir2::NEG_X),
            Self::Left => Some(Dir2::Y),
            Self::Right => Some(Dir2::NEG_Y),
        }
    }

    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();
        input_map.insert(Self::Up, KeyCode::ArrowUp);
        input_map.insert(Self::Up, KeyCode::KeyW);
        input_map.insert(Self::Up, GamepadButton::DPadUp);

        input_map.insert(Self::Down, KeyCode::ArrowDown);
        input_map.insert(Self::Down, KeyCode::KeyS);
        input_map.insert(Self::Down, GamepadButton::DPadDown);

        input_map.insert(Self::Left, KeyCode::ArrowLeft);
        input_map.insert(Self::Left, KeyCode::KeyA);
        input_map.insert(Self::Left, GamepadButton::DPadLeft);

        input_map.insert(Self::Right, KeyCode::ArrowRight);
        input_map.insert(Self::Right, KeyCode::KeyD);
        input_map.insert(Self::Right, GamepadButton::DPadRight);

        input_map
    }
}

#[derive(Resource, Default)]
struct CursorCoordinates(Vec2);

fn project_cursor(
    mut cursor_coordinates: ResMut<CursorCoordinates>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera.single();
    let window = window.single();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Z)) else {
        return;
    };
    let global_cursor = ray.get_point(distance);
    cursor_coordinates.0 = global_cursor.xy();
}

fn move_player(
    mut query: Query<(&mut Transform, &ActionState<Action>), With<Player>>,
    cursor_coordinates: Res<CursorCoordinates>,
    time: Res<Time>,
) {
    for (mut transform, action) in query.iter_mut() {
        let mut movement = Vec2::ZERO;
        for direction in Action::DIRECTIONS {
            if action.pressed(&direction) {
                if let Some(contribution) = direction.direction() {
                    movement += *contribution;
                }
            }
        }
        movement = movement.normalize_or_zero() * time.delta_secs();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
        transform.rotation =
            Quat::from_rotation_z((transform.translation.xy() - cursor_coordinates.0).to_angle());
    }
}
