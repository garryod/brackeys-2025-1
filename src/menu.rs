use crate::{cleanup, AppState};
use bevy::{
    app::{AppExit, Plugin, Update},
    color::Color,
    prelude::{
        default, in_state, BuildChildren, Button, Camera3d, Changed, ChildBuild, Commands,
        Component, EventWriter, IntoSystemConfigs, NextState, OnEnter, OnExit, Query, ResMut, Text,
        With,
    },
    text::TextFont,
    ui::{
        AlignItems, BackgroundColor, FlexDirection, Interaction, JustifyContent, Node, UiRect, Val,
    },
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::Menu), setup)
            .add_systems(OnExit(AppState::Menu), cleanup::<Menu>)
            .add_systems(Update, action.run_if(in_state(AppState::Menu)));
    }
}

#[derive(Component)]
struct Menu;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
}

fn setup(mut commands: Commands) {
    let button_node = Node {
        width: Val::Px(300.),
        height: Val::Px(65.),
        margin: UiRect::all(Val::Px(20.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_background = BackgroundColor(Color::BLACK);
    let button_font = TextFont {
        font_size: 33.,
        ..default()
    };

    commands.spawn((Menu, Camera3d::default()));
    commands
        .spawn((
            Menu,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,

                ..default()
            },
            BackgroundColor(Color::hsl(200., 0.5, 0.5)),
        ))
        .with_children(|menu| {
            menu.spawn((
                Text::new("Brackeys 2025.1"),
                Node {
                    margin: UiRect::all(Val::Px(50.)),
                    ..default()
                },
                TextFont {
                    font_size: 67.,
                    ..default()
                },
            ));
            menu.spawn((
                Button,
                button_node.clone(),
                button_background,
                MenuButtonAction::Play,
            ))
            .with_children(|play_button| {
                play_button.spawn((Text::new("New Game"), button_font.clone()));
            });
            menu.spawn((
                Button,
                button_node.clone(),
                button_background,
                MenuButtonAction::Quit,
            ))
            .with_children(|play_button| {
                play_button.spawn((Text::new("Quit"), button_font.clone()));
            });
        });
}

#[allow(clippy::type_complexity)]
fn action(
    query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<AppState>>,
    mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, button_action) in &query {
        if *interaction == Interaction::Pressed {
            match button_action {
                MenuButtonAction::Play => {
                    app_state.set(AppState::Game);
                }
                MenuButtonAction::Quit => {
                    app_exit.send(AppExit::Success);
                }
            }
        }
    }
}
