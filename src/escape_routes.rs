use bevy::prelude::*;
use crate::route_events::StartRoute;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct EscapeRoute {
    pub id: usize,
}

pub struct EscapeRoutePlugin;

impl Plugin for EscapeRoutePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_debug_routes)
           .add_systems(Update, check_escape_collision);
    }
}

fn spawn_debug_routes(mut commands: Commands) {
    // Debug trigger squares
    let positions = [
        (1, Vec2::new(-400., 250.)),
        (2, Vec2::new(-300., -200.)),
        (3, Vec2::new(100., 220.)),
        (4, Vec2::new(250., -50.)),
        (5, Vec2::new(420., 120.)),
        (6, Vec2::new(-10., -300.)),
    ];
    for (id, pos) in positions {
        commands.spawn((
            EscapeRoute { id },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.7, 0.2 * id as f32, 0.3),
                    custom_size: Some(Vec2::splat(40.)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, 0.),
                ..default()
            },
        ));
    }
    info!("Debug escape route markers spawned.");
}

fn check_escape_collision(
    player_q: Query<&Transform, With<Player>>,
    routes_q: Query<(&Transform, &EscapeRoute)>,
    mut ev_route: EventWriter<StartRoute>,
) {
    let Ok(player_t) = player_q.get_single() else { return; };
    let player_pos = player_t.translation.truncate();
    for (t, route) in routes_q.iter() {
        let d = player_pos.distance(t.translation.truncate());
        if d < 48.0 {
            ev_route.send(StartRoute { route_id: route.id });
        }
    }
}