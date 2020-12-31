use std::collections::BinaryHeap;
use kira::{
    sequence::Sequence,
    Duration,
};

use midly::{TrackEvent, TrackEventKind, MidiMessage, Timing, Smf, Format, MetaMessage};

pub mod sound_bank;

#[derive(PartialEq, Eq)]
struct EventAtTime<'a>(i32, &'a TrackEvent<'a>);
impl std::cmp::Ord for EventAtTime<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl std::cmp::PartialOrd for EventAtTime<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((-self.0).cmp(&-other.0))
    }
}
pub fn sequence_from_midi(midi: &Smf, instruments: &sound_bank::Instruments, transpose: i32 ) -> Sequence {
    let mut sequence = Sequence::<()>::new(Default::default());
    let mut ticks_per_beat = 1;
    if midi.header.format == Format::Sequential {
        panic!("sequential SMFs not currently supported");
    }
    match midi.header.timing {
        Timing::Metrical(v) => {
            ticks_per_beat = v.as_int() as u32;
        }
        Timing::Timecode(fps, subframes) => {
            panic!("Only metrical timing is currently supported");
        }
    }

    let mut events = BinaryHeap::new();
    for track in &midi.tracks {
        let mut time = 0;
        for event in track {
            time += event.delta.as_int() as i32;
            events.push(EventAtTime(time, event));
        }
    }
    sequence.start_loop();
    let mut time = 0;
    for EventAtTime(event_time, event) in events {
        let delta = event_time - time;
        time = event_time;
        match event.kind {
            TrackEventKind::Midi { channel, message } => {
                if delta > 0 {
                    sequence.wait_for_interval(delta as f64 / ticks_per_beat as f64);
                }
                match message {
                    MidiMessage::NoteOn { key, vel } => {
                        if let Some(sound_bank) = instruments.sound_bank_for_channel(channel.as_int() as u32) {
                            let key = key.as_int() as i32 + transpose;
                            if let Some(sound_id) = sound_bank.sound_id_for_note(key as u32) {
                                sequence.play(sound_id, Default::default());
                            } else {
                                println!("woops: {}", key);
                            }
                        }
                    }
                    MidiMessage::NoteOff { key, .. } => {
                        if let Some(sound_bank) = instruments.sound_bank_for_channel(channel.as_int() as u32) {
                            let key = key.as_int() as i32 + transpose;
                            if let Some(sound_id) = sound_bank.sound_id_for_note(key as u32) {
                                sequence.stop_instances_of(sound_id, Default::default());
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    sequence
}
