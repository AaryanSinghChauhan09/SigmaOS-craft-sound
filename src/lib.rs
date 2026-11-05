#[derive(Debug, Clone)]
pub struct AudioTrack { pub name: String, pub sample_rate: u32, pub muted: bool }
impl AudioTrack {
    pub fn new(name: impl Into<String>, rate: u32) -> Self {
        Self { name: name.into(), sample_rate: rate, muted: false }
    }
}

#[derive(Debug, Default)]
pub struct DawSession { pub name: String, tracks: Vec<AudioTrack> }
impl DawSession {
    pub fn new(name: impl Into<String>) -> Self { Self { name: name.into(), tracks: Vec::new() } }
    pub fn add(&mut self, t: AudioTrack) { self.tracks.push(t); }
    pub fn count(&self) -> usize { self.tracks.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_daw() {
        let mut d = DawSession::new("test");
        d.add(AudioTrack::new("Vocal", 48000));
        assert_eq!(d.count(), 1);
    }
}
