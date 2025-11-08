use anyhow::Context;
use bevy::prelude::*;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize, Clone)]
pub struct Timeline {
    pub title: String,
    pub frames: Vec<Frame>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Frame {
    pub index: usize,
    pub time: String,     // e.g., "00:00–00:06" (en dash or hyphen supported)
    pub camera: String,   // Camera & Action
    pub lighting: String, // Lighting
    #[serde(default)]
    pub notes: String, // VO/SFX Notes
}

impl Timeline {
    pub fn frame_duration_secs(&self, idx: usize) -> f32 {
        if let Some(f) = self.frames.get(idx) {
            let (a, b) = parse_time_range(&f.time);
            (b - a).max(0.01)
        } else {
            0.0
        }
    }
}

#[derive(Resource)]
pub struct ActiveTimeline {
    pub timeline: Timeline,
    pub current: usize,
    pub timer: Timer,
}

impl ActiveTimeline {
    pub fn from_timeline(t: &Timeline) -> Self {
        let duration = t.frame_duration_secs(0);
        Self {
            timeline: t.clone(),
            current: 0,
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }

    pub fn current_frame(&self) -> Option<&Frame> {
        self.timeline.frames.get(self.current)
    }

    pub fn tick_and_maybe_advance(&mut self, delta: Duration) -> bool {
        self.timer.tick(delta);
        if self.timer.finished() {
            self.current += 1;
            if self.current < self.timeline.frames.len() {
                let secs = self.timeline.frame_duration_secs(self.current);
                self.timer = Timer::from_seconds(secs, TimerMode::Once);
                true
            } else {
                // End of timeline; keep None frame
                false
            }
        } else {
            false
        }
    }
}

pub fn load_timeline_from_yaml_str(yaml: &str) -> anyhow::Result<Timeline> {
    let t: Timeline = serde_yaml::from_str(yaml).context("Parsing YAML timeline")?;
    // Basic validation: indexes monotonic and time ranges sane
    for (i, f) in t.frames.iter().enumerate() {
        if f.index != i + 1 {
            anyhow::bail!("Frame index mismatch at position {}: got {}", i, f.index);
        }
        let (a, b) = parse_time_range(&f.time);
        if b <= a {
            anyhow::bail!("Non-positive duration at frame {} time '{}'", f.index, f.time);
        }
    }
    Ok(t)
}

// Accepts "mm:ss–mm:ss" or "mm:ss-mm:ss" or with spaces
fn parse_time_range(s: &str) -> (f32, f32) {
    let cleaned = s.trim().replace('–', "-"); // normalize en-dash to hyphen
    let mut parts = cleaned.split('-').map(str::trim);
    let left = parts.next().unwrap_or("00:00");
    let right = parts.next().unwrap_or("00:01");
    (parse_mmss(left), parse_mmss(right))
}

fn parse_mmss(s: &str) -> f32 {
    let mut parts = s.split(':');
    let m: f32 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
    let sec: f32 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
    m * 60.0 + sec
}