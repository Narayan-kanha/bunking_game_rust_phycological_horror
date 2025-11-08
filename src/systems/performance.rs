use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, Diagnostics};

pub struct PerformancePlugin;

#[derive(Component)]
struct FpsText;

impl Plugin for PerformancePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
           .add_systems(Startup, spawn_fps_display)
           .add_systems(Update, fps_update_system);
    }
}

fn spawn_fps_display(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle {
            text: Text::from_section("FPS: --", TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 12.0, color: Color::WHITE }),
            style: Style { position_type: PositionType::Absolute, top: Val::Px(6.0), right: Val::Px(6.0), ..default() },
            ..default()
        },
        FpsText,
    ));
}

fn fps_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    if let Some(fps) = diagnostics.get(bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(avg) = fps.average() {
            for mut text in &mut query {
                text.sections[0].value = format!("FPS: {:.1}", avg);
            }
        }
    }
}
