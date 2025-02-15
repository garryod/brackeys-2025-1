use bevy::{
    app::{Plugin, Startup, Update},
    diagnostic::{
        DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    input::ButtonInput,
    prelude::{
        default, BuildChildren, Commands, Component, Entity, KeyCode, Node, Query, Res, Text,
        TextUiWriter, Visibility, With,
    },
    ui::{PositionType, UiRect, Val},
};

pub struct FpsPlugin {
    pub toggle_key: KeyCode,
}

impl Default for FpsPlugin {
    fn default() -> Self {
        Self {
            toggle_key: KeyCode::F12,
        }
    }
}

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(EntityCountDiagnosticsPlugin)
            .add_plugins(SystemInformationDiagnosticsPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, (update, toggle(self.toggle_key)));
    }
}

#[derive(Component)]
struct FpsRoot;

#[derive(Component)]
struct FpsText;

fn setup(mut commands: Commands) {
    commands
        .spawn((
            FpsRoot,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Percent(1.),
                top: Val::Percent(1.),
                bottom: Val::Auto,
                left: Val::Auto,
                padding: UiRect::all(Val::Px(4.)),
                ..default()
            },
        ))
        .with_child((FpsText, Text::new("FPS: N/A")));
}

fn update(
    mut query: Query<Entity, With<FpsText>>,
    mut writer: TextUiWriter,
    diagnostics: Res<DiagnosticsStore>,
) {
    if let Some(fps) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    {
        for text in &mut query {
            *writer.text(text, 0) = format!("FPS: {fps:.0}");
        }
    }
}

fn toggle(
    toggle_key: KeyCode,
) -> impl FnMut(Query<&mut Visibility, With<FpsRoot>>, Res<ButtonInput<KeyCode>>) {
    move |mut query: Query<&mut Visibility, With<FpsRoot>>, keyboard: Res<ButtonInput<KeyCode>>| {
        if keyboard.just_pressed(toggle_key) {
            for mut visibility in &mut query {
                *visibility = match *visibility {
                    Visibility::Hidden => Visibility::Visible,
                    _ => Visibility::Hidden,
                }
            }
        }
    }
}
