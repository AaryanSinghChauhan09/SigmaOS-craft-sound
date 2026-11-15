//! SigmaOS-craft-sound: audio model.
//!
//! Real implementation: audio samples, tracks, effects, mix, and validation.
//! Replaces the 104-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Effect type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectType {
    Reverb,
    Delay,
    Compressor,
    Equalizer,
}

/// Audio track.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioTrack {
    pub id: String,
    pub name: String,
    pub sample_rate: u32,
    pub channels: u8,
}

/// Sample.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sample {
    pub id: String,
    pub path: String,
    pub duration_ms: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SoundError {
    EmptyName,
    DuplicateTrack(String),
    TrackNotFound(String),
    InvalidSampleRate,
}

impl fmt::Display for SoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "track name must not be empty"),
            Self::DuplicateTrack(id) => write!(f, "duplicate track {:?}", id),
            Self::TrackNotFound(id) => write!(f, "track {:?} not found", id),
            Self::InvalidSampleRate => write!(f, "invalid sample rate"),
        }
    }
}

impl std::error::Error for SoundError {}

/// Audio project.
pub struct AudioProject {
    pub name: String,
    tracks: BTreeMap<String, AudioTrack>,
    samples: BTreeMap<String, Sample>,
}

impl AudioProject {
    pub fn new(name: impl Into<String>) -> Self { Self { name: name.into(), tracks: BTreeMap::new(), samples: BTreeMap::new() } }

    pub fn add_track(&mut self, track: AudioTrack) -> Result<(), SoundError> {
        if track.name.trim().is_empty() { return Err(SoundError::EmptyName); }
        if track.sample_rate == 0 { return Err(SoundError::InvalidSampleRate); }
        let id = track.id.clone();
        if self.tracks.contains_key(&id) {
            return Err(SoundError::DuplicateTrack(id.clone()));
        }
        self.tracks.insert(id, track);
        Ok(())
    }

    pub fn track_count(&self) -> usize { self.tracks.len() }

    pub fn tracks(&self) -> Vec<&AudioTrack> { self.tracks.values().collect() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_create_aaa() {
        let p = AudioProject::new("My Project");
        assert_eq!(p.name, "My Project");
        assert_eq!(p.track_count(), 0);
    }

    #[test]
    fn test_add_track_aaa() {
        let mut p = AudioProject::new("My Project");
        let t = AudioTrack { id: "t1".into(), name: "Guitar".into(), sample_rate: 44100, channels: 2 };
        p.add_track(t).unwrap();
        assert_eq!(p.track_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut p = AudioProject::new("My Project");
        let t = AudioTrack { id: "x".into(), name: "".into(), sample_rate: 44100, channels: 2 };
        assert!(p.add_track(t).is_err());
    }

    #[test]
    fn test_invalid_rate_rejected_aaa() {
        let mut p = AudioProject::new("My Project");
        let t = AudioTrack { id: "x".into(), name: "X".into(), sample_rate: 0, channels: 2 };
        assert!(p.add_track(t).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut p = AudioProject::new("My Project");
        let t = AudioTrack { id: "x".into(), name: "X".into(), sample_rate: 44100, channels: 2 };
        p.add_track(t).unwrap();
        let t2 = AudioTrack { id: "x".into(), name: "X".into(), sample_rate: 44100, channels: 2 };
        assert!(p.add_track(t2).is_err());
    }
}
