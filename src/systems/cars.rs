use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::data::assets_loader::Handles;
use crate::config::{CAR_SPAWN_RADIUS, SIMULATION_RADIUS};
use crate::core::player::Ethan;
use crate::core::world::MissionEntity;

#[derive(Component)]
pub struct Car {
    pub speed: f32,
    pub direction: Vec3,
}

#[derive(Resource, Default)]
pub struct CarSpawner {
    pub spawn_points: Vec<Vec3>,
    pub max_cars: usize,
    pub last_spawn: f64,
}

pub struct CarsPlugin;

impl Plugin for CarsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CarSpawner>()
           .add_systems(OnEnter(crate::states::GameState::Mission1), init_spawner)
           .add_systems(Update, car_spawn_system.run_if(in_state(crate::states::GameState::Mission1)))
           .add_systems(Update, car_ai_system.run_if(in_state(crate::states::GameState::Mission1)));
    }
}

fn init_spawner(mut spawner: ResMut<CarSpawner>) {
    for i in -3..=3 {
        let offset = i as f32 * 40.0;
        spawner.spawn_points.push(Vec3::new(-380.0, 0.5, offset));
        spawner.spawn_points.push(Vec3::new(380.0, 0.5, offset));
        spawner.spawn_points.push(Vec3::new(offset, 0.5, -380.0));
        spawner.spawn_points.push(Vec3::new(offset, 0.5, 380.0));
    }
    spawner.max_cars = 24;
}

fn car_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut spawner: ResMut<CarSpawner>,
    player_q: Query<&Transform, With<Ethan>>,
    handles: Res<Handles>,
    existing: Query<&Car>,
) {
    let Ok(player_tf) = player_q.get_single() else { return };
    let player_pos = player_tf.translation;

    let active_count = existing.iter().count();
    if active_count >= spawner.max_cars { return; }

    if time.elapsed_seconds_f64() - spawner.last_spawn < 0.6 { return; }
    spawner.last_spawn = time.elapsed_seconds_f64();

    for spawn in &spawner.spawn_points {
        let dist = spawn.distance(player_pos);
        if dist < CAR_SPAWN_RADIUS && dist > 10.0 {
            // avoid spawning in front of player roughly
            let forward = player_tf.forward();
            let to_spawn = (spawn - player_pos).normalize();
            if forward.dot(to_spawn) < -0.15 {
                let dir = if spawn.x.abs() > spawn.z.abs() { Vec3::new(-spawn.x.signum(), 0.0, 0.0) } else { Vec3::new(0.0, 0.0, -spawn.z.signum()) };
                // spawn Kenney car model if available else box
                if let Some(car_scene) = &handles.kenney_car {
                    commands.spawn((
                        SceneBundle { scene: car_scene.clone(), transform: Transform::from_translation(*spawn).with_scale(Vec3::splat(0.7)), ..default() },
                        Car { speed: 6.0 + rand::random::<f32>() * 3.0, direction: dir },
                        RigidBody::KinematicPositionBased,
                        Collider::cuboid(0.8, 0.4, 1.6),
                        MissionEntity,
                    ));
                } else {
                    commands.spawn((
                        PbrBundle { mesh: Mesh::from(shape::Box::new(1.6, 0.8, 3.2)).into(), material: Default::default(), transform: Transform::from_translation(*spawn), ..default() },
                        Car { speed: 6.0 + rand::random::<f32>() * 3.0, direction: dir },
                        RigidBody::KinematicPositionBased,
                        Collider::cuboid(0.8, 0.4, 1.6),
                        MissionEntity,
                    ));
                }
                break;
            }
        }
    }
}

fn car_ai_system(
    time: Res<Time>,
    mut cars: Query<(&mut Transform, &Car)>,
    player_q: Query<&Transform, With<Ethan>>,
) {
    let Ok(player) = player_q.get_single() else { return };
    let player_pos = player.translation;

    for (mut tf, car) in &mut cars {
        let d = tf.translation.distance(player_pos);
        if d > SIMULATION_RADIUS { continue; } // skip far-away cars
        let mv = car.direction.normalize() * car.speed * time.delta_seconds();
        tf.translation += mv;
        // small randomness
        if rand::random::<f32>() < 0.0015 {
            let angle = (rand::random::<f32>() - 0.5) * 0.6;
            let rot = Quat::from_axis_angle(Vec3::Y, angle);
            let mut dir = car.direction;
            dir = rot.mul_vec3(dir);
            tf.rotation = Quat::from_rotation_y(0.0);
        }
    }
}
