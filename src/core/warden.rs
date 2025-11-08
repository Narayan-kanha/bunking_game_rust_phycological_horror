use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::states::GameState;

#[derive(Component)]
pub struct TheWarden {
    pub waypoints: Vec<Vec3>,
    pub current: usize,
    pub vision_range: f32,
    pub fov_deg: f32,
}

pub struct WardenPlugin;

impl Plugin for WardenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Mission1), spawn_warden)
           .add_systems(Update, warden_ai_system.run_if(in_state(GameState::Mission1)));
    }
}

fn spawn_warden(mut commands: Commands, handles: Res<crate::data::assets_loader::Handles>) {
    let waypoints = vec![
        Vec3::new(-12.0, 0.0, -6.0),
        Vec3::new(12.0, 0.0, -6.0),
        Vec3::new(12.0, 0.0, 8.0),
        Vec3::new(-12.0, 0.0, 8.0),
    ];

    // spawn model if available
    commands.spawn((
        if handles.punk.is_loaded() {
            SceneBundle {
                scene: handles.punk.clone(),
                transform: Transform::from_translation(waypoints[0]),
                ..default()
            }
        } else {
            SceneBundle {
                scene: handles.punk.clone(), // fallback same
                transform: Transform::from_translation(waypoints[0]),
                ..default()
            }
        },
        TheWarden {
            waypoints,
            current: 0,
            vision_range: 12.0,
            fov_deg: 70.0,
        },
        RigidBody::KinematicPositionBased,
        Collider::capsule_y(0.8, 0.25),
    ));
}

fn warden_ai_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut TheWarden)>,
    player_q: Query<&Transform, With<crate::core::player::Ethan>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok((mut tf, mut warden)) = query.get_single_mut() else { return };
    let Ok(player_tf) = player_q.get_single() else { return };

    let to_player = player_tf.translation - tf.translation;
    let dist = to_player.length();
    if dist <= warden.vision_range {
        // simple direct spot
        let dir = to_player.normalize();
        let forward = tf.forward();
        let angle = forward.dot(dir).clamp(-1.0,1.0).acos().to_degrees();
        if angle <= warden.fov_deg * 0.5 {
            next_state.set(GameState::Caught);
            return;
        }
    }

    // patrol
    let target = warden.waypoints[warden.current];
    let mut dir_to = target - tf.translation;
    dir_to.y = 0.0;
    let d = dir_to.length();
    if d > 0.3 {
        let speed = 2.0;
        tf.translation += dir_to.normalize() * speed * time.delta_seconds();
        tf.look_at(target, Vec3::Y);
    } else {
        warden.current = (warden.current + 1) % warden.waypoints.len();
    }
}
