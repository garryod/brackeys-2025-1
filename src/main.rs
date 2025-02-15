mod fps_counter;

use crate::fps_counter::FpsPlugin;
use bevy::{
    app::{App, Startup},
    prelude::{Camera3d, Commands},
    DefaultPlugins,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FpsPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}
