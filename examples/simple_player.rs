use regex::Regex;

use kira::manager::AudioManager;
use midly::Smf;

use midi_renderer::{
    sound_bank::SoundBank,
    sequence_from_midi,
};

fn main() {
    let mut audio_manager = AudioManager::new(Default::default()).unwrap();

    // This assumes sound files with file names in the format used by https://philharmonia.co.uk/resources/sound-samples/
    let sample_set = SoundBank::from_directory(&"sound_bank", &Regex::new(r".*/bass-clarinet_(?P<note>[A-G]s?)(?P<octave>[0-9])_025_fortissimo_normal.mp3").unwrap(), &mut audio_manager);

    // Try this midi file: https://upload.wikimedia.org/wikipedia/commons/9/9d/Drunken_sailor.mid
    let data = std::fs::read("Drunken_sailor.mid").unwrap();
    let smf = Smf::parse(&data).unwrap();

    let sequence = sequence_from_midi(&smf, 1, &sample_set, 0);

    audio_manager.start_sequence(sequence, Default::default()).unwrap();
    audio_manager.start_metronome().unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
