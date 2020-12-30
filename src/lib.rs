use kira::{
    sequence::Sequence,
};

use midly::{TrackEvent, TrackEventKind, MidiMessage, Timing, Smf};

pub mod sound_bank;

pub fn sequence_from_midi(midi: &Smf, track: usize, sound_bank: &sound_bank::SoundBank, transpose: i32) -> Sequence {
    let mut sequence = Sequence::<()>::new(Default::default());
    match midi.header.timing {
        Timing::Metrical(v) => {
            let tpm = v.as_int() as f64 * 90.0;
            println!("ticks per minute: {}", tpm);
            sequence.set_metronome_tempo(kira::Tempo::from(tpm));
        },
        Timing::Timecode(..) => panic!()
    }

    sequence.start_loop();
    for event in &midi.tracks[track] {
        if let TrackEvent { kind: TrackEventKind::Midi { channel, message }, delta } = event {
            if delta.as_int() > 0 {
                sequence.wait_for_interval(delta.as_int() as f64);
            }
            match message {
                MidiMessage::NoteOn { key, vel } => {
                    let key = key.as_int() as i32 + transpose;
                    if let Some(sound_id) = sound_bank.sound_id_for_note(key as u32) {
                        sequence.play(sound_id, Default::default());
                    } else {
                        println!("woops: {}", key);
                    }
                }
                MidiMessage::NoteOff { key, .. } => {
                    let key = key.as_int() as i32 + transpose;
                    if let Some(sound_id) = sound_bank.sound_id_for_note(key as u32) {
                        sequence.stop_instances_of(sound_id.into(), Default::default());
                    }
                }
                _ => {}
            }
        }
    }

    sequence
}
