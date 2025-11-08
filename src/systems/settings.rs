use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;
use crate::config::SAVE_DIR;
use crate::utils::ensure_dir;

#[derive(Resource, Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub low_spec_mode: bool,
    pub shadow_quality: u8,
    pub max_particles: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            low_spec_mode: false,
            shadow_quality: 2,
            max_particles: 200,
        }
    }
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        let s = load_settings();
        app.insert_resource(s);
    }
}

fn load_settings() -> Settings {
    ensure_dir("config");
    let path = "config/settings.json";
    if let Ok(json) = fs::read_to_string(path) {
        if let Ok(s) = serde_json::from_str::<Settings>(&json) {
            return s;
        }
    }
    Settings::default()
}

fn save_settings(settings: Res<Settings>) {
    if let Ok(json) = serde_json::to_string_pretty(&*settings) {
        let _ = fs::write("config/settings.json", json);
    }
}
