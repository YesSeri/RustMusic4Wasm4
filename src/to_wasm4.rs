use crate::Alter;
use crate::MusicNote;
enum TonePulse {
    One,
    Two,
    Triangle,
}
enum Note {
    A,
    Ab,
    B,
    C,
    D,
    E,
    F,
    G,
}
fn note_to_wasm4(note: MusicNote, tp: TonePulse) -> String {
    "tone(440, 60, 100, TONE_PULSE1)".to_string()
}

#[cfg(test)]
mod tests {
    use crate::{
        to_wasm4::{note_to_wasm4, TonePulse},
        Alter, MusicNote,
    };
    #[test]
    fn it_works() {
        let m = MusicNote::new("A".to_string(), 4, Alter::Natural);
        assert_eq!(
            note_to_wasm4(m, TonePulse::One),
            "tone(440, 60, 100, TONE_PULSE1)".to_string()
        );
        // tone(262, 60, 100, TONE_PULSE1);
    }
}
