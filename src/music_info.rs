use crate::music_note::MusicNote;

// Contains info about one beat.
#[derive(Debug)]
pub struct MusicInfo {
    pub note: Option<MusicNote>,
    pub duration: usize,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // assert_eq!(
        //     note_to_wasm4(m, TonePulse::One),
        //     "tone(440, 60, 100, TONE_PULSE1)".to_string()
        // );
    }
}
