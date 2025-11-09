use anyhow::{Context, Result};
use bevy::prelude::*;
use serde::Deserialize;
use std::time::Duration;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Timeline {
    pub title: String,
    pub frames: Vec<Frame>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Frame {
    pub index: usize,
    pub time: String,     // "00:00–00:06"
    pub camera: String,
    pub lighting: String,
    #[serde(default)]
    pub notes: String,
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
    pub finished: bool,
}

impl ActiveTimeline {
    pub fn from_timeline(t: &Timeline) -> Self {
        let duration = t.frame_duration_secs(0);
        Self {
            timeline: t.clone(),
            current: 0,
            timer: Timer::from_seconds(duration, TimerMode::Once),
            finished: false,
        }
    }

    pub fn current_frame(&self) -> Option<&Frame> {
        if self.finished {
            None
        } else {
            self.timeline.frames.get(self.current)
        }
    }

    pub fn tick_and_maybe_advance(&mut self, delta: Duration) -> bool {
        if self.finished {
            return false;
        }
        self.timer.tick(delta);
        if self.timer.finished() {
            self.current += 1;
            if self.current < self.timeline.frames.len() {
                let secs = self.timeline.frame_duration_secs(self.current);
                self.timer = Timer::from_seconds(secs, TimerMode::Once);
                true
            } else {
                self.finished = true;
                false
            }
        } else {
            false
        }
    }
}

pub fn load_timeline_from_yaml_str(yaml: &str) -> Result<Timeline> {
    let t: Timeline = serde_yaml::from_str(yaml).context("Parsing YAML timeline")?;
    validate_timeline(&t)?;
    Ok(t)
}

pub fn load_timeline_from_file(path: &str) -> Result<Timeline> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Reading timeline file: {}", path))?;
    load_timeline_from_yaml_str(&content)
}

fn validate_timeline(t: &Timeline) -> Result<()> {
    for (i, f) in t.frames.iter().enumerate() {
        if f.index != i + 1 {
            anyhow::bail!("Frame index mismatch at position {}: got {}", i, f.index);
        }
        let (a, b) = parse_time_range(&f.time);
        if b <= a {
            anyhow::bail!("Non-positive duration at frame {} time '{}'", f.index, f.time);
        }
    }
    Ok(())
}

// Accepts "mm:ss–mm:ss" or "mm:ss-mm:ss"
fn parse_time_range(s: &str) -> (f32, f32) {
    let cleaned = s.trim().replace('–', "-");
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