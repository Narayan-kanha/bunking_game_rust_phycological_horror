use bevy::prelude::*;

mod narrative;

use narrative::{load_timeline_from_yaml_str, ActiveTimeline, Timeline};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Path1TrueWake,
    Path6SunkLegend,
}

#[derive(Component)]
struct TimelineEntity;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.02, 0.02, 0.03)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Freshman Roll — Prototype".to_string(),
                resolution: (1280., 720.).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        // Menu
        .add_systems(Startup, setup_menu_camera)
        .add_systems(Update, menu_input.run_if(in_state(GameState::Menu)))
        // Path 1
        .add_systems(OnEnter(GameState::Path1TrueWake), path1_load)
        .add_systems(
            Update,
            run_timeline.run_if(in_state(GameState::Path1TrueWake)),
        )
        .add_systems(OnExit(GameState::Path1TrueWake), cleanup_timeline)
        // Path 6
        .add_systems(OnEnter(GameState::Path6SunkLegend), path6_load)
        .add_systems(
            Update,
            run_timeline.run_if(in_state(GameState::Path6SunkLegend)),
        )
        .add_systems(OnExit(GameState::Path6SunkLegend), cleanup_timeline)
        .run();
}

fn setup_menu_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(),));
    info!("Freshman Roll — Prototype Booted");
    info!("Controls:");
    info!("  [1] Start Path 1 — THE TRUE WAKE");
    info!("  [6] Start Path 6 — THE SUNK LEGEND");
    info!("  [Esc] Return to Menu");
}

fn menu_input(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Digit1) {
        next_state.set(GameState::Path1TrueWake);
    }
    if keyboard.just_pressed(KeyCode::Digit6) {
        next_state.set(GameState::Path6SunkLegend);
    }
}

static PATH1_YAML: &str = include_str!("../assets/narrative/path1_true_wake.yaml");
static PATH6_YAML: &str = include_str!("../assets/narrative/path6_sunk_legend.yaml");

fn path1_load(mut commands: Commands) {
    let timeline: Timeline = match load_timeline_from_yaml_str(PATH1_YAML) {
        Ok(t) => t,
        Err(err) => {
            error!("Failed to load Path 1 timeline: {err:?}");
            return;
        }
    };
    spawn_timeline(&mut commands, timeline, "Path 1 — THE TRUE WAKE");
}

fn path6_load(mut commands: Commands) {
    let timeline: Timeline = match load_timeline_from_yaml_str(PATH6_YAML) {
        Ok(t) => t,
        Err(err) => {
            error!("Failed to load Path 6 timeline: {err:?}");
            return;
        }
    };
    spawn_timeline(&mut commands, timeline, "Path 6 — THE SUNK LEGEND");
}

fn spawn_timeline(commands: &mut Commands, timeline: Timeline, label: &str) {
    info!("===== {label} =====");
    info!("Frames: {}", timeline.frames.len());
    commands.insert_resource(ActiveTimeline::from_timeline(&timeline));
    // A background quad to show frame changes via color
    commands.spawn((
        TimelineEntity,
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
}

fn run_timeline(
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
    mut active: Option<ResMut<ActiveTimeline>>,
    mut bg_query: Query<&mut Sprite, With<TimelineEntity>>,
    state: Res<State<GameState>>,
) {
    let Some(mut active) = active.as_mut() else {
        return;
    };

    // Tick and advance frames
    let just_advanced = active.tick_and_maybe_advance(time.delta());

    if let Some(frame) = active.current_frame() {
        if just_advanced {
            // Visualize progress by coloring the background deterministically per frame index
            if let Ok(mut sprite) = bg_query.get_single_mut() {
                let c = color_for_index(frame.index);
                sprite.color = c;
            }

            info!(
                "Frame {:02}: {}  |  Time: {}  |  Light: {}  |  Notes: {}",
                frame.index, frame.camera, frame.time, frame.lighting, frame.notes
            );
        }
    } else {
        info!("Timeline complete.");
        // Return to menu after completion
        next_state.set(GameState::Menu);
        // Drop resource so OnExit cleanup handles entities
    }

    // Allow escape to abort back to menu
    // We read keyboard through input resource in this system for brevity:
    // Note: direct input access avoided here; if desired, add a separate system gated by state
    if false {
        // placeholder to keep pattern visible
        // We rely on menu_input for state switching in Menu
    }

    // On completion, the resource is left until state actually changes; cleanup in OnExit
    // (This avoids mutability issues when changing states during a system)
    // No-op here
    // The next_state is already set above when done
}

fn color_for_index(i: usize) -> Color {
    // Simple hash to color
    let r = (((i as f32) * 37.0) % 255.0) / 255.0;
    let g = (((i as f32) * 83.0) % 255.0) / 255.0;
    let b = (((i as f32) * 149.0) % 255.0) / 255.0;
    Color::rgb(r.max(0.15), g.max(0.15), b.max(0.15))
}

fn cleanup_timeline(
    mut commands: Commands,
    q_timeline_entities: Query<Entity, With<TimelineEntity>>,
) {
    for e in q_timeline_entities.iter() {
        commands.entity(e).despawn_recursive();
    }
    commands.remove_resource::<ActiveTimeline>();
    info!("Cleaned up timeline entities and state.");
}