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
use endings::GameEnding;
use escape_routes::{EscapeRoutePlugin, Player};
use route_mapping::{route_timeline_path, route_result_ending};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GamePhase {
    #[default]
    Menu,
    InTimeline,
    FinishedTimeline,
}

#[derive(Component)]
struct TimelineBackdrop;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.02, 0.02, 0.03)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Freshman Roll — Route Prototype".into(),
                resolution: (1280., 720.).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_state::<GamePhase>()
        .insert_resource(GameProgress::default())
        // Events
        .add_event::<StartRoute>()
        .add_event::<EndingCompleted>()
        .add_event::<FinalBellUnlocked>()
        // Plugins
        .add_plugin(EscapeRoutePlugin)
        // Startup
        .add_systems(Startup, setup_menu_camera)
        .add_systems(Startup, spawn_player)
        // Menu input
        .add_systems(Update, menu_input.run_if(in_state(GamePhase::Menu)))
        // Route start & timeline load
        .add_systems(Update, on_start_route.run_if(in_state(GamePhase::Menu)))
        // Timeline progression
        .add_systems(
            Update,
            run_timeline.run_if(in_state(GamePhase::InTimeline)),
        )
        // Handle timeline completion → register ending
        .add_systems(Update, check_timeline_finished.run_if(in_state(GamePhase::InTimeline)))
        // Progression / unlock monitoring
        .add_systems(Update, progression_monitor)
        .run();
}

fn setup_menu_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    info!("Freshman Roll — Route Prototype Booted");
    info!("Walk onto colored squares to trigger different escape routes.");
}

fn spawn_player(mut commands: Commands) {
    // Temporary player marker (replace with your existing entity)
    commands.spawn((
        Player,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.8, 0.8, 0.2),
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
    mut next: ResMut<NextState<GamePhase>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        info!("Already in menu.");
    }
    // Optional manual trigger for debug: press F1 to list completed endings
    if keyboard.just_pressed(KeyCode::F1) {
        info!("Press W/A/S/D to move player onto a route trigger.");
        next.set(GamePhase::Menu);
    }
}

fn on_start_route(
    mut ev: EventReader<StartRoute>,
    mut next: ResMut<NextState<GamePhase>>,
    mut commands: Commands,
) {
    for StartRoute { route_id } in ev.iter() {
        if let Some(path) = route_timeline_path(*route_id) {
            match load_timeline_from_file(path) {
                Ok(timeline) => {
                    info!("Starting timeline for route {} -> {}", route_id, path);
                    commands.insert_resource(ActiveTimeline::from_timeline(&timeline));
                    // Backdrop entity
                    commands.spawn((
                        TimelineBackdrop,
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::rgb(0.05, 0.06, 0.08),
                                custom_size: Some(Vec2::new(5000., 5000.)),
                                ..default()
                            },
                            transform: Transform::from_xyz(0., 0., -10.),
                            ..default()
                        },
                    ));
                    next.set(GamePhase::InTimeline);
                }
                Err(err) => {
                    error!("Failed to load timeline for route {}: {:?}", route_id, err);
                }
            }
        } else {
            warn!("No mapping for route {}", route_id);
        }
    }
}

fn run_timeline(
    time: Res<Time>,
    mut active: Option<ResMut<ActiveTimeline>>,
    mut backdrop_q: Query<&mut Sprite, With<TimelineBackdrop>>,
) {
    let Some(mut active) = active.as_mut() else { return; };
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
        info!("Timeline finished (waiting for completion system).");
    }
}

fn check_timeline_finished(
    mut active: Option<ResMut<ActiveTimeline>>,
    mut next: ResMut<NextState<GamePhase>>,
    mut commands: Commands,
    backdrop_q: Query<Entity, With<TimelineBackdrop>>,
    mut ending_ev: EventWriter<EndingCompleted>,
) {
    let Some(active_tl) = active.as_mut() else { return; };
    if active_tl.finished {
        // Determine ending based on last loaded route — reconstruct by title or store route id.
        // Simplest: infer from title keywords (requires consistent naming)
        let inferred = infer_ending_from_title(&active_tl.timeline.title);
        if let Some(ending) = inferred {
            ending_ev.send(EndingCompleted { ending });
            info!("Registered ending: {:?}", ending);
        } else {
            warn!("Could not infer ending from title '{}'", active_tl.timeline.title);
        }

        // Cleanup
        if let Ok(e) = backdrop_q.get_single() {
            commands.entity(e).despawn_recursive();
        }
        commands.remove_resource::<ActiveTimeline>();
        next.set(GamePhase::Menu);
    }
}

fn infer_ending_from_title(title: &str) -> Option<GameEnding> {
    use GameEnding::*;
    let t = title.to_ascii_lowercase();
    if t.contains("true wake") { Some(TrueWake) }
    else if t.contains("cycle breaker") { Some(CycleBreaker) }
    else if t.contains("legend") && !t.contains("sunk") { Some(Legend) }
    else if t.contains("puppetmaster") { Some(Puppetmaster) }
    else if t.contains("fragmented") { Some(FragmentedMind) }
    else if t.contains("sunk legend") { Some(SunkLegend) }
    else { None }
}

fn progression_monitor(
    mut gp: ResMut<GameProgress>,
    mut ending_ev: EventReader<EndingCompleted>,
    mut unlock_ev: EventWriter<FinalBellUnlocked>,
) {
    for e in ending_ev.iter() {
        gp.mark_completed(e.ending);
        info!("Progress: {:?} endings completed.", gp.completed.len());
        if gp.final_bell_unlocked {
            unlock_ev.send(FinalBellUnlocked);
        }
    }
}

fn color_for_index(i: usize) -> Color {
    let r = (((i as f32) * 37.0) % 255.0) / 255.0;
    let g = (((i as f32) * 83.0) % 255.0) / 255.0;
    let b = (((i as f32) * 149.0) % 255.0) / 255.0;
    Color::rgb(r.max(0.15), g.max(0.15), b.max(0.15))

}