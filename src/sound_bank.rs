use std::{
    path::Path,
    collections::HashMap,
};
use regex::Regex;

use kira::{
    sound::SoundId,
    manager::AudioManager,
};

pub struct SoundBank {
    samples: HashMap<u32, SoundId>,
}

impl SoundBank {
    pub fn from_directory(path: impl AsRef<Path>, pattern: &Regex, audio_manager: &mut AudioManager) -> Self {
        let mut samples = vec![];
        for entry in std::fs::read_dir(path.as_ref()).unwrap() {
            let entry = entry.unwrap();
            if let Some(captures) = pattern.captures(entry.path().to_str().unwrap()) {
                let note = match captures
                    .name("note")
                    .unwrap()
                    .as_str()
                    .to_uppercase()
                    .as_str()
                {
                    "C" => 0,
                    "CS" => 1,
                    "C#" => 1,
                    "D" => 2,
                    "DS" => 3,
                    "D#" => 3,
                    "E" => 4,
                    "F" => 5,
                    "FS" => 6,
                    "F#" => 6,
                    "G" => 7,
                    "GS" => 8,
                    "G#" => 8,
                    "A" => 9,
                    "AS" => 10,
                    "A#" => 10,
                    "B" => 11,
                    _ => panic!(),
                };
                let octave = captures
                    .name("octave")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();
                let adjusted_note = note + (octave + 1) * 12;
                samples.push((
                    adjusted_note,
                    audio_manager.load_sound(entry.path(), Default::default()).unwrap(),
                ));
            }
        }
        Self { samples: samples.into_iter().collect() }
    }

    pub fn sound_id_for_note(&self, note: u32) -> Option<SoundId> {
        self.samples.get(&note).cloned()
    }
}
