use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod core;
mod systems;
mod data;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::srgba(0.04, 0.04, 0.06, 1.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bunk Again: Lakshmi Narayan".to_string(),
                resolution: (1600.0, 900.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            WorldInspectorPlugin::default(),
        ))
        .add_systems(Startup, core::world::setup_world)
        .add_systems(Update, (
            systems::ui::update_ui,
            systems::cars::spawn_cars,
        ))
        .run();
}
