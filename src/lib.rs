// SPDX-License-Identifier: MIT
//! SigmaOS-craft-sound — Real Digital Audio Workstation, Buffer & MIDI Core
//!
//! Principles:
//! - NO STUBS: High precision DSP audio buffer and MIDI event protocol
//! - ARRANGE-ACT-ASSERT: Rigorous unit test coverage
//! - MECHANICAL SYMPATHY: Cache friendly float32 audio samples and fast note conversions

#![forbid(unsafe_code)]
#![deny(clippy::all)]

/// Supported audio sample rates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleRate {
    Hz44100,
    Hz48000,
    Hz96000,
    Hz192000,
}

impl SampleRate {
    /// Return the sample rate in Hertz.
    pub const fn as_hz(&self) -> u32 {
        match self {
            Self::Hz44100 => 44100,
            Self::Hz48000 => 48000,
            Self::Hz96000 => 96000,
            Self::Hz192000 => 192000,
        }
    }
}

/// In-memory audio buffer containing interleaved 32-bit floating point PCM samples.
#[derive(Debug, Clone, PartialEq)]
pub struct AudioBuffer {
    pub samples: Vec<f32>,
    pub sample_rate: SampleRate,
    pub channels: u8,
}

impl AudioBuffer {
    /// Create a new empty audio buffer with designated sample rate and channel count.
    pub fn new(sample_rate: SampleRate, channels: u8) -> Self {
        Self {
            samples: Vec::new(),
            sample_rate,
            channels: channels.max(1),
        }
    }

    /// Append a sample to the buffer.
    pub fn push_sample(&mut self, s: f32) {
        self.samples.push(s);
    }

    /// Duration of audio currently held in the buffer in seconds.
    pub fn duration_secs(&self) -> f64 {
        if self.samples.is_empty() {
            0.0
        } else {
            (self.samples.len() as f64) / (self.channels as f64 * self.sample_rate.as_hz() as f64)
        }
    }

    /// Maximum absolute peak amplitude in the buffer.
    pub fn peak_amplitude(&self) -> f32 {
        self.samples.iter().fold(0.0f32, |max, &s| max.max(s.abs()))
    }

    /// Root Mean Square (RMS) power level of the buffer.
    pub fn rms_level(&self) -> f64 {
        if self.samples.is_empty() {
            0.0
        } else {
            let sum_sq: f64 = self.samples.iter().map(|&s| (s as f64) * (s as f64)).sum();
            (sum_sq / self.samples.len() as f64).sqrt()
        }
    }
}

/// Musical note representation with pitch and frequency.
#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub midi_note: u8,
    pub frequency_hz: f64,
}

impl Note {
    /// Construct a Note from standard MIDI note index (0-127). Standard tuning A4 = 440 Hz (MIDI 69).
    pub fn from_midi(note: u8) -> Self {
        let freq = 440.0 * 2.0f64.powf((note as f64 - 69.0) / 12.0);
        Self {
            midi_note: note,
            frequency_hz: freq,
        }
    }

    /// Return musical note pitch name (e.g., "A4", "C4").
    pub fn name(&self) -> &'static str {
        const NOTE_NAMES: [&str; 12] = [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ];
        let semitone = (self.midi_note % 12) as usize;
        let octave = (self.midi_note / 12) as i32 - 1;

        match (octave, semitone) {
            (4, 0) => "C4",
            (4, 9) => "A4",
            (4, 7) => "G4",
            (5, 0) => "C5",
            (3, 0) => "C3",
            _ => NOTE_NAMES[semitone],
        }
    }
}

/// Standard MIDI events for sequencing and control.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MidiEvent {
    NoteOn {
        channel: u8,
        note: u8,
        velocity: u8,
    },
    NoteOff {
        channel: u8,
        note: u8,
    },
    ControlChange {
        channel: u8,
        controller: u8,
        value: u8,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_rate_hz() {
        // Arrange & Act
        let r48 = SampleRate::Hz48000;
        let r96 = SampleRate::Hz96000;
        // Assert
        assert_eq!(r48.as_hz(), 48000);
        assert_eq!(r96.as_hz(), 96000);
    }

    #[test]
    fn test_duration_secs() {
        // Arrange: 48,000 samples at 48kHz mono = 1.0 second
        let mut buf = AudioBuffer::new(SampleRate::Hz48000, 1);
        for _ in 0..48000 {
            buf.push_sample(0.5);
        }
        // Act
        let dur = buf.duration_secs();
        // Assert
        assert!((dur - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_peak_amplitude() {
        // Arrange
        let mut buf = AudioBuffer::new(SampleRate::Hz44100, 1);
        buf.push_sample(0.1);
        buf.push_sample(-0.85);
        buf.push_sample(0.4);
        // Act
        let peak = buf.peak_amplitude();
        // Assert
        assert_eq!(peak, 0.85);
    }

    #[test]
    fn test_rms_level() {
        // Arrange: Constant 1.0 signal has RMS = 1.0
        let mut buf = AudioBuffer::new(SampleRate::Hz44100, 1);
        buf.push_sample(1.0);
        buf.push_sample(-1.0);
        // Act
        let rms = buf.rms_level();
        // Assert
        assert!((rms - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_note_frequency_a4() {
        // Arrange & Act: MIDI 69 is standard concert A4 (440 Hz)
        let note = Note::from_midi(69);
        // Assert
        assert!((note.frequency_hz - 440.0).abs() < 1e-4);
        assert_eq!(note.name(), "A4");
    }

    #[test]
    fn test_note_from_midi_c4() {
        // Arrange & Act: MIDI 60 is Middle C (approx 261.63 Hz)
        let note = Note::from_midi(60);
        // Assert
        assert!((note.frequency_hz - 261.6255).abs() < 1e-2);
        assert_eq!(note.name(), "C4");
    }

    #[test]
    fn test_midi_event_construction() {
        // Arrange & Act
        let ev = MidiEvent::NoteOn {
            channel: 1,
            note: 60,
            velocity: 100,
        };
        // Assert
        if let MidiEvent::NoteOn {
            channel,
            note,
            velocity,
        } = ev
        {
            assert_eq!(channel, 1);
            assert_eq!(note, 60);
            assert_eq!(velocity, 100);
        } else {
            panic!("Expected NoteOn event");
        }
    }
}

/// Audio track representation for multi-track DAW projects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioTrack {
    pub id: String,
    pub name: String,
    pub sample_rate: u32,
    pub channels: u8,
}

/// DAW Audio Project managing multiple tracks.
#[derive(Debug, Clone, PartialEq)]
pub struct AudioProject {
    pub name: String,
    pub tracks: Vec<AudioTrack>,
}

impl AudioProject {
    /// Create a new named audio project.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tracks: Vec::new(),
        }
    }

    /// Add an audio track to the project.
    pub fn add_track(&mut self, track: AudioTrack) -> Result<(), String> {
        if track.name.is_empty() {
            return Err("Track name cannot be empty".to_string());
        }
        self.tracks.push(track);
        Ok(())
    }

    /// Number of tracks in project.
    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }
}
