use bevy::prelude::*;

mod narrative;
mod endings;
mod progression;
mod route_events;
mod escape_routes;
mod route_mapping;

use narrative::{ActiveTimeline, load_timeline_from_file};
use route_events::{StartRoute, EndingCompleted, FinalBellUnlocked};
use progression::GameProgress;
use escape_routes::{EscapeRoutePlugin, Player};
use route_mapping::{route_timeline_path, route_result_ending};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GamePhase {
    #[default]
    Menu,
    InTimeline,
}

#[derive(Resource)]
struct ActiveRoute {
    id: usize,
}

#[derive(Component)]
struct TimelineBackdrop;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.03)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Freshman Roll — Route Prototype".into(),
                resolution: (1280., 720.).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .insert_state(GamePhase::Menu)
        .insert_resource(GameProgress::default())
        .add_event::<StartRoute>()
        .add_event::<EndingCompleted>()
        .add_event::<FinalBellUnlocked>()
        .add_plugins(EscapeRoutePlugin)
        .add_systems(Startup, setup_menu_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, menu_input.run_if(in_state(GamePhase::Menu)))
        .add_systems(Update, on_start_route.run_if(in_state(GamePhase::Menu)))
        .add_systems(Update, run_timeline.run_if(in_state(GamePhase::InTimeline)))
        .add_systems(Update, check_timeline_finished.run_if(in_state(GamePhase::InTimeline)))
        .add_systems(Update, progression_monitor)
        .run();
}

fn setup_menu_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    info!("Freshman Roll — Route Prototype Booted");
    info!("Walk onto colored squares to trigger escape routes.");
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.8, 0.8, 0.2),
                custom_size: Some(Vec2::new(30., 30.)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 10.),
            ..default()
        },
    ));
}

fn menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        info!("Debug: F1 pressed (menu is active).");
    }
}

fn on_start_route(
    mut ev: EventReader<StartRoute>,
    mut next: ResMut<NextState<GamePhase>>,
    mut commands: Commands,
) {
    for start in ev.read() {
        let route_id = start.route_id;
        if let Some(path) = route_timeline_path(route_id) {
            match load_timeline_from_file(path) {
                Ok(timeline) => {
                    info!("Starting timeline for route {} -> {}", route_id, path);
                    info!("Timeline: {}", timeline.title); // Use title to silence narrative warning
                    commands.insert_resource(ActiveTimeline::from_timeline(&timeline));
                    commands.insert_resource(ActiveRoute { id: route_id });
                    commands.spawn((
                        TimelineBackdrop,
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::srgb(0.05, 0.06, 0.08),
                                custom_size: Some(Vec2::new(5000., 5000.)),
                                ..default()
                            },
                            transform: Transform::from_xyz(0., 0., -10.),
                            ..default()
                        },
                    ));
                    next.set(GamePhase::InTimeline);
                }
                Err(err) => error!("Failed to load timeline for route {}: {:?}", route_id, err),
            }
        } else {
            warn!("No timeline mapping for route {}", route_id);
        }
    }
}

fn run_timeline(
    time: Res<Time>,
    mut active: Option<ResMut<ActiveTimeline>>,
    mut backdrop_q: Query<&mut Sprite, With<TimelineBackdrop>>,
) {
    let Some(active) = active.as_mut() else { return; };
    let just_advanced = active.tick_and_maybe_advance(time.delta());
    if let Some(frame) = active.current_frame() {
        if just_advanced {
            if let Ok(mut sprite) = backdrop_q.get_single_mut() {
                sprite.color = color_for_index(frame.index);
            }
            info!(
                "Frame {:02}: {} | Time {} | Light {} | Notes {}",
                frame.index, frame.camera, frame.time, frame.lighting, frame.notes
            );
        }
    } else if active.finished {
        info!("Timeline finished.");
    }
}

fn check_timeline_finished(
    active: Option<Res<ActiveTimeline>>,
    route: Option<Res<ActiveRoute>>,
    mut next: ResMut<NextState<GamePhase>>,
    mut commands: Commands,
    backdrop_q: Query<Entity, With<TimelineBackdrop>>,
    mut ending_ev: EventWriter<EndingCompleted>,
) {
    let Some(active_tl) = active else { return; };
    if !active_tl.finished {
        return;
    }
    if let Some(route_res) = route {
        if let Some(ending) = route_result_ending(route_res.id) {
            ending_ev.send(EndingCompleted { ending });
            info!("Registered ending: {:?}", ending);
        } else {
            warn!("No ending mapped for route {}", route_res.id);
        }
    } else {
        warn!("Missing ActiveRoute resource at timeline end.");
    }

    if let Ok(e) = backdrop_q.get_single() {
        commands.entity(e).despawn_recursive();
    }
    commands.remove_resource::<ActiveTimeline>();
    commands.remove_resource::<ActiveRoute>();
    next.set(GamePhase::Menu);
}

fn progression_monitor(
    mut gp: ResMut<GameProgress>,
    mut ending_ev: EventReader<EndingCompleted>,
    mut unlock_ev: EventWriter<FinalBellUnlocked>,
) {
    for ev in ending_ev.read() {
        gp.mark_completed(ev.ending);
        info!("Progress: {} endings completed.", gp.completed.len());
        if gp.final_bell_unlocked {
            unlock_ev.send(FinalBellUnlocked);
        }
    }
}

fn color_for_index(i: usize) -> Color {
    let r = (((i as f32) * 37.0) % 255.0) / 255.0;
    let g = (((i as f32) * 83.0) % 255.0) / 255.0;
    let b = (((i as f32) * 149.0) % 255.0) / 255.0;
    Color::srgb(r.max(0.15), g.max(0.15), b.max(0.15))

}