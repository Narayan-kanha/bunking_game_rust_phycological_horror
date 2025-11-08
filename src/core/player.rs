use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::systems::controls::Controls;
use crate::systems::inventory::Inventory;

#[derive(Component)]
pub struct Ethan;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(crate::states::GameState::Mission1), spawn_player)
           .add_systems(Update, player_input_system.run_if(in_state(crate::states::GameState::Mission1)));
    }
}

#[derive(Component)]
pub struct PlayerController;

#[derive(Resource, Default, Clone)]
pub struct PlayerStats {
    pub health: f32,
    pub max_health: f32,
    pub hunger: f32,
    pub max_hunger: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            hunger: 100.0,
            max_hunger: 100.0,
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    handles: Res<crate::data::assets_loader::Handles>,
) {
    let start = Vec3::new(0.0, 0.0, 0.0);

    // spawn the player scene if available, otherwise spawn a capsule
    if handles.man.is_loaded() {
        commands.spawn((
            SceneBundle {
                scene: handles.man.clone(),
                transform: Transform::from_translation(start).with_scale(Vec3::splat(1.0)),
                ..default()
            },
            Ethan,
            PlayerController,
            RigidBody::KinematicPositionBased,
            Collider::capsule_y(0.9, 0.35),
            LockedAxes::ROTATION_LOCKED,
        ));
    } else {
        commands.spawn((
            PbrBundle {
                mesh: Mesh::from(shape::Capsule { radius: 0.35, depth: 1.8 }).into(),
                material: Default::default(),
                transform: Transform::from_translation(start),
                ..default()
            },
            Ethan,
            PlayerController,
            RigidBody::KinematicPositionBased,
            Collider::capsule_y(0.9, 0.35),
            LockedAxes::ROTATION_LOCKED,
        ));
    }

    commands.insert_resource(PlayerStats::default());
    info!("Player spawned.");
}

fn player_input_system(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    controls: Res<Controls>,
    mut q: Query<&mut Transform, With<Ethan>>,
    mut stats: ResMut<PlayerStats>,
) {
    let Ok(mut tf) = q.get_single_mut() else { return };

    let mut dir = Vec3::ZERO;
    if keyboard.pressed(controls.move_forward) { dir.z -= 1.0; }
    if keyboard.pressed(controls.move_back) { dir.z += 1.0; }
    if keyboard.pressed(controls.move_left) { dir.x -= 1.0; }
    if keyboard.pressed(controls.move_right) { dir.x += 1.0; }

    if dir != Vec3::ZERO {
        let speed = 6.0;
        tf.translation += dir.normalize() * speed * time.delta_seconds();
        tf.look_to(-dir, Vec3::Y);
        // hunger depletes while moving
        stats.hunger = (stats.hunger - 0.6 * time.delta_seconds()).clamp(0.0, stats.max_hunger);
    } else {
        // some regen
        stats.hunger = (stats.hunger + 0.05 * time.delta_seconds()).clamp(0.0, stats.max_hunger);
    }

    // if hunger zero, gradually reduce health
    if stats.hunger <= 0.0 {
        stats.health = (stats.health - 2.0 * time.delta_seconds()).max(0.0);
    }
}
