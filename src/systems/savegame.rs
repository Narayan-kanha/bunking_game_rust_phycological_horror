use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use crate::config::{SAVE_DIR, AUTOSAVE_FILE, DEFAULT_SAVE_EXT};
use crate::utils::ensure_dir;
use crate::systems::inventory::{Inventory, ItemType, ItemStack};

#[derive(Serialize, Deserialize)]
struct SaveData {
    player_pos: [f32; 3],
    inventory: Vec<(ItemType, u32, Option<i32>)>,
    mission_progress: u8,
    health: f32,
    hunger: f32,
}

pub struct SaveGamePlugin;

impl Plugin for SaveGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (save_input_system, load_input_system));
        ensure_dir(SAVE_DIR);
    }
}

fn save_path(name: &str) -> PathBuf {
    ensure_dir(SAVE_DIR);
    let mut p = PathBuf::from(SAVE_DIR);
    let filename = if name.ends_with(DEFAULT_SAVE_EXT) { name.to_string() } else { format!("{}{}", name, DEFAULT_SAVE_EXT) };
    p.push(filename);
    p
}

fn save_input_system(
    keyboard: Res<Input<KeyCode>>,
    player_q: Query<&Transform, With<crate::core::player::Ethan>>,
    inv: Res<Inventory>,
    stats: Option<Res<crate::core::player::PlayerStats>>,
) {
    if keyboard.just_pressed(KeyCode::F5) {
        if let Ok(t) = player_q.get_single() {
            let items = inv.slots.iter().map(|(k, s)| (k.clone(), s.count, s.durability)).collect::<Vec<_>>();
            let data = SaveData {
                player_pos: [t.translation.x, t.translation.y, t.translation.z],
                inventory: items,
                mission_progress: inv.mission_progress,
                health: stats.as_ref().map(|s| s.health).unwrap_or(100.0),
                hunger: stats.as_ref().map(|s| s.hunger).unwrap_or(100.0),
            };
            let path = save_path(AUTOSAVE_FILE);
            if let Ok(json) = serde_json::to_string_pretty(&data) {
                if fs::write(&path, json).is_ok() {
                    info!("Saved game to {}", path.display());
                } else {
                    error!("Failed to write save file");
                }
            }
        }
    }
}

fn load_input_system(
    keyboard: Res<Input<KeyCode>>,
    mut player_q: Query<&mut Transform, With<crate::core::player::Ethan>>,
    mut inv: ResMut<Inventory>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::F9) {
        let path = save_path(AUTOSAVE_FILE);
        if let Ok(json) = fs::read_to_string(&path) {
            if let Ok(data) = serde_json::from_str::<SaveData>(&json) {
                if let Ok(mut t) = player_q.get_single_mut() {
                    t.translation = Vec3::new(data.player_pos[0], data.player_pos[1], data.player_pos[2]);
                }
                inv.slots.clear();
                for (k, count, dur) in data.inventory {
                    inv.slots.insert(k.clone(), ItemStack { item: k, count, durability: dur });
                }
                inv.mission_progress = data.mission_progress;
                if let Some(mut stats) = commands.get_resource_mut::<crate::core::player::PlayerStats>() {
                    stats.health = data.health;
                    stats.hunger = data.hunger;
                }
                info!("Loaded game from {}", path.display());
            }
        } else {
            info!("No save file found at {}", path.display());
        }
    }
}
