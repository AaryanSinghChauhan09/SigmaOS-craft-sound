use sigmaos_craft_sound::{AudioProject, AudioTrack};

#[test]
fn audio_project_integration_aaa() {
    let mut p = AudioProject::new("My Project");
    p.add_track(AudioTrack {
        id: "t1".into(),
        name: "Guitar".into(),
        sample_rate: 44100,
        channels: 2,
    })
    .unwrap();
    assert_eq!(p.track_count(), 1);
}
