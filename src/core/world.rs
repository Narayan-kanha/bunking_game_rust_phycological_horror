use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::data::assets_loader::Handles;
use crate::core::npc_ai::NpcCategory;
use crate::config::{CAR_SPAWN_RADIUS, SIMULATION_RADIUS};
use crate::core::npc_ai::Npc; // component
use crate::core::npc_ai::NpcState;

#[derive(Component)]
pub struct MissionEntity;

/// Resource listing important spawn points for cars and NPCs
#[derive(Resource, Default)]
pub struct WorldSpawnInfo {
    pub car_spawn_points: Vec<Vec3>,
    pub npc_spawn_points: Vec<Vec3>,
    pub market_spots: Vec<Vec3>,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldSpawnInfo::default())
           .add_systems(Startup, setup_world);
    }
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut spawn_info: ResMut<WorldSpawnInfo>,
    handles: Res<Handles>,
) {
    info!("Setting up the open world...");

    // GROUND
    let ground_material = mats.add(Color::rgb(0.12, 0.45, 0.12).into());
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 1600.0 })),
            material: ground_material,
            ..default()
        },
        Collider::cuboid(800.0, 0.1, 800.0),
        RigidBody::Fixed,
        MissionEntity,
    ));

    // PRIMARY SCHOOL BUILDING (maze-like)
    spawn_school(&mut commands, &mut meshes, &mut mats);

    // ROADS GRID
    spawn_road_grid(&mut commands, &mut meshes, &mut mats, &mut spawn_info);

    // FOOD CLUSTERS + MARKET SPOTS
    spawn_food_and_market(&mut commands, &mut meshes, &mut mats, &handles, &mut spawn_info);

    // NPC spawn points (distributed near market/roads/park)
    generate_npc_spawn_points(&mut spawn_info);

    // Optionally: spawn a few NPCs immediately to populate the world
    spawn_initial_npcs(&mut commands, &handles, &spawn_info);

    // Car spawn points are created in spawn_road_grid (used by CarsPlugin/car spawner)
    info!("World setup complete: {} NPC spawn points, {} car spawn points", spawn_info.npc_spawn_points.len(), spawn_info.car_spawn_points.len());
}

fn spawn_school(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, mats: &mut ResMut<Assets<StandardMaterial>>) {
    let wall_mat = mats.add(StandardMaterial::from(Color::rgb(0.88, 0.88, 0.9)));
    let floor_mat = mats.add(StandardMaterial::from(Color::rgb(0.9, 0.9, 0.84)));

    // outer shell
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(120.0, 12.5, 90.0))),
            material: wall_mat.clone(),
            transform: Transform::from_translation(Vec3::new(40.0, 6.25, 0.0)),
            ..default()
        },
        MissionEntity,
    ));

    // inner corridors / rooms
    for rx in -4..=4 {
        for rz in -3..=3 {
            let room_pos = Vec3::new(40.0 + rx as f32 * 9.0, 0.0, rz as f32 * 9.0);
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(7.5, 4.0, 7.5))),
                    material: floor_mat.clone(),
                    transform: Transform::from_translation(room_pos + Vec3::Y * 2.0),
                    ..default()
                },
                MissionEntity,
            ));
            // small lockers / obstacles
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(0.9, 2.0, 0.5))),
                    material: wall_mat.clone(),
                    transform: Transform::from_translation(room_pos + Vec3::new(-2.2, 1.0, 0.0)),
                    ..default()
                },
                MissionEntity,
            ));
        }
    }
}

fn spawn_road_grid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    mats: &mut ResMut<Assets<StandardMaterial>>,
    spawn_info: &mut WorldSpawnInfo,
) {
    let road_mat = mats.add(StandardMaterial::from(Color::rgb(0.08, 0.08, 0.08)));
    // grid of roads centered around origin
    for i in -5..=5 {
        // horizontal roads (long)
        let z = i as f32 * 40.0;
        let road_ent = commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(1600.0, 0.2, 8.0))),
                material: road_mat.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, 0.05, z)),
                ..default()
            },
            MissionEntity,
        )).id();

        // create spawn points at both ends of the road segment for cars
        spawn_info.car_spawn_points.push(Vec3::new(-780.0, 0.5, z));
        spawn_info.car_spawn_points.push(Vec3::new(780.0, 0.5, z));
    }

    for i in -5..=5 {
        let x = i as f32 * 40.0;
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(8.0, 0.2, 1600.0))),
                material: road_mat.clone(),
                transform: Transform::from_translation(Vec3::new(x, 0.05, 0.0)),
                ..default()
            },
            MissionEntity,
        ));
        spawn_info.car_spawn_points.push(Vec3::new(x, 0.5, -780.0));
        spawn_info.car_spawn_points.push(Vec3::new(x, 0.5, 780.0));
    }
}

fn spawn_food_and_market(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    mats: &mut ResMut<Assets<StandardMaterial>>,
    handles: &Handles,
    spawn_info: &mut WorldSpawnInfo,
) {
    // market spots near a road intersection
    let market_positions = vec![
        Vec3::new(20.0, 0.0, -20.0),
        Vec3::new(-28.0, 0.0, 24.0),
        Vec3::new(60.0, 0.0, 40.0),
    ];

    for (i, pos) in market_positions.iter().enumerate() {
        // spawn a simple stall using kenney assets if available
        if let Some(market_scene) = &handles.kenney_market {
            commands.spawn((
                SceneBundle {
                    scene: market_scene.clone(),
                    transform: Transform::from_translation(*pos).with_scale(Vec3::splat(0.9)),
                    ..default()
                },
                MissionEntity,
            ));
        } else {
            // simple stall fallback
            let stall_mat = mats.add(StandardMaterial::from(Color::rgb(0.7, 0.45, 0.3)));
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(2.0, 1.2, 2.0))),
                    material: stall_mat,
                    transform: Transform::from_translation(*pos + Vec3::Y * 0.6),
                    ..default()
                },
                MissionEntity,
            ));
        }
        spawn_info.market_spots.push(*pos);
    }

    // Food pickup clusters (small items) — use kenney food if available
    let food_positions = vec![Vec3::new(18.0, 0.0, -18.0), Vec3::new(-30.0, 0.0, 22.0), Vec3::new(62.0, 0.0, 42.0)];
    for pos in food_positions {
        if let Some(food_scene) = &handles.kenney_food {
            commands.spawn((
                SceneBundle {
                    scene: food_scene.clone(),
                    transform: Transform::from_translation(pos + Vec3::Y * 0.2).with_scale(Vec3::splat(0.6)),
                    ..default()
                },
                crate::systems::inventory::FoodPickup,
                Collider::ball(0.4),
                RigidBody::Fixed,
                MissionEntity,
            ));
        } else {
            // fallback sphere
            let food_mat = mats.add(StandardMaterial::from(Color::rgb(0.9, 0.6, 0.2)));
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.4, sectors: 8, stacks: 6 })),
                    material: food_mat,
                    transform: Transform::from_translation(pos + Vec3::Y * 0.45),
                    ..default()
                },
                crate::systems::inventory::FoodPickup,
                Collider::ball(0.4),
                RigidBody::Fixed,
                MissionEntity,
            ));
        }
    }
}

/// generate NPC spawn points distributed around market & roads
fn generate_npc_spawn_points(spawn_info: &mut WorldSpawnInfo) {
    // near market
    for x in -2..=2 {
        for z in -2..=2 {
            spawn_info.npc_spawn_points.push(Vec3::new(20.0 + x as f32 * 2.2, 0.0, -20.0 + z as f32 * 2.2));
        }
    }
    // along roads
    for i in -4..=4 {
        let z = i as f32 * 40.0;
        spawn_info.npc_spawn_points.push(Vec3::new(6.0, 0.0, z));
    }
    // some scattered around school perimeter
    spawn_info.npc_spawn_points.push(Vec3::new(36.0, 0.0, -30.0));
    spawn_info.npc_spawn_points.push(Vec3::new(46.0, 0.0, 12.0));
    spawn_info.npc_spawn_points.push(Vec3::new(12.0, 0.0, 52.0));
}

/// spawn a handful of NPCs immediately to populate the world (random types)
fn spawn_initial_npcs(commands: &mut Commands, handles: &Handles, spawn_info: &WorldSpawnInfo) {
    use rand::prelude::*;
    let mut rng = rand::thread_rng();

    // choose up to N random spawn points
    let picks = spawn_info.npc_spawn_points.iter().cloned().take(24).collect::<Vec<_>>();
    for pos in picks {
        let roll: f32 = rng.gen();
        let category = if roll < 0.12 {
            NpcCategory::Trader
        } else if roll < 0.45 {
            NpcCategory::Civilian
        } else {
            NpcCategory::Student
        };

        let mut builder = commands.spawn((
            TransformBundle::from_transform(Transform::from_translation(pos + Vec3::Y * 0.0)),
            Npc { category: category.clone(), state: NpcState::Idle, timer: 0.0 },
            MissionEntity,
        ));

        // spawn model: prefer different glbs to diversify
        let model_handle = match category {
            NpcCategory::Trader => handles.worker.clone().into(),
            NpcCategory::Civilian => handles.man.clone().into(),
            NpcCategory::Student => handles.animated_woman.clone().unwrap_or_else(|| handles.man.clone()).into(),
        };

        // if model is loaded spawn scene; if not, fallback to a capsule
        builder.insert((
            SceneBundle { scene: model_handle, transform: Transform::IDENTITY.with_scale(Vec3::splat(0.9)), ..default() }
        ));
    }
}
