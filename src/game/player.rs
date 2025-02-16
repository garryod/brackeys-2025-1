use bevy::{
    app::{Plugin, Update},
    asset::Assets,
    color::Color,
    math::{Dir2, Vec2},
    pbr::{MeshMaterial3d, StandardMaterial},
    prelude::{
        in_state, Bundle, Component, Cuboid, GamepadButton, IntoSystemConfigs, KeyCode, Mesh,
        Mesh3d, Query, Res, ResMut, Transform, With,
    },
    reflect::Reflect,
    time::Time,
};
use leafwing_input_manager::{
    plugin::InputManagerPlugin,
    prelude::{ActionState, InputMap},
    Actionlike,
};

use crate::AppState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(Update, move_player.run_if(in_state(AppState::Game)));
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    action_state: ActionState<Action>,
    input_map: InputMap<Action>,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
}

impl PlayerBundle {
    pub fn new(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        Self {
            player: Player,
            action_state: ActionState::<Action>::default(),
            input_map: Action::default_input_map(),
            mesh: Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 2.))),
            material: MeshMaterial3d(materials.add(Color::hsl(220., 0.1, 0.5))),
        }
    }
}

#[derive(Component)]
struct Player;

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

fn move_player(
    mut query: Query<(&mut Transform, &ActionState<Action>), With<Player>>,
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
    }
}
