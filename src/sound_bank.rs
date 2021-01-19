use std::{
    path::Path,
    collections::HashMap,
};
use regex::Regex;

use kira::{
    sound::handle::SoundHandle,
    manager::AudioManager,
};

pub struct Instruments {
    pub channels: HashMap<u32, SoundBank>,
    pub default: Option<u32>,
}

impl Instruments {
    pub fn sound_bank_for_channel(&self, channel: u32) -> Option<&SoundBank> {
        if let Some(bank) = self.channels.get(&channel) {
            Some(bank)
        } else if let Some(default_channel) = self.default {
            self.channels.get(&default_channel)
        } else {
            None
        }
    }
}

pub struct SoundBank {
    samples: HashMap<u32, SoundHandle>,
}

impl SoundBank {
    pub fn from_directory(path: impl AsRef<Path>, pattern: &Regex, audio_manager: &mut AudioManager) -> Self {
        let mut samples = vec![];
        for entry in std::fs::read_dir(path.as_ref()).unwrap() {
            let entry = entry.unwrap();
            if let Some(captures) = pattern.captures(entry.path().to_str().unwrap()) {
                let note = if let Some(note) = captures.name("note") {
                     let note = match note
                        .as_str()
                        .to_uppercase()
                        .as_str()
                    {
                        "C" => 0,
                        "CS" => 1,
                        "C#" => 1,
                        "DB" => 1,
                        "D" => 2,
                        "DS" => 3,
                        "D#" => 3,
                        "EB" => 3,
                        "E" => 4,
                        "F" => 5,
                        "FS" => 6,
                        "F#" => 6,
                        "GB" => 6,
                        "G" => 7,
                        "GS" => 8,
                        "G#" => 8,
                        "AB" => 8,
                        "A" => 9,
                        "AS" => 10,
                        "A#" => 10,
                        "BB" => 10,
                        "B" => 11,
                        _ => panic!(),
                    };
                    let octave = captures
                        .name("octave")
                        .unwrap()
                        .as_str()
                        .parse::<u32>()
                        .unwrap();
                    note + (octave + 1) * 12
                } else if let Some(midi_note) = captures.name("midi_note") {
                    midi_note.as_str().parse::<u32>().unwrap()
                } else {
                    panic!("Must supply either a standard notation note or a midi note number");
                };

                samples.push((
                    note,
                    audio_manager.load_sound(entry.path(), Default::default()).unwrap(),
                ));
            }
        }
        Self { samples: samples.into_iter().collect() }
    }

    pub fn sound_id_for_note(&self, note: u32) -> Option<SoundHandle> {
        self.samples.get(&note).cloned()
    }
}
