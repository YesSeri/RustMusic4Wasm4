use crate::{alter::Alter, note::Note};

#[derive(Debug)]
pub struct MusicNote {
    pub pitch: Note,
    pub octave: usize,
}
impl MusicNote {
    pub fn new(step: String, octave: usize, alter: Alter) -> Self {
        MusicNote {
            pitch: Note::get_note(step, alter),
            octave,
        }
    }
    pub fn get_hz(&self) -> u32 {
        let hz_without_octave: u32 = self.pitch.into();
        match self.octave {
            8 => hz_without_octave * 16,
            7 => hz_without_octave * 8,
            6 => hz_without_octave * 4,
            5 => hz_without_octave * 2,
            4 => hz_without_octave,
            3 => hz_without_octave / 2,
            2 => hz_without_octave / 4,
            1 => hz_without_octave / 8,
            0 => hz_without_octave / 16,
            _ => unreachable!(),
        }
    }
}
impl From<MusicNote> for String {
    fn from(m: MusicNote) -> Self {
        format!("tone({}, 60, 100, TONE_PULSE1)", m.get_hz())
        // "tone(440, 60, 100, TONE_PULSE1)".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Alter, MusicNote};
    #[test]
    fn octave_test() {
        let m = MusicNote::new("A".to_string(), 5, Alter::Natural);
        let s: String = m.into();
        assert_eq!(s, "tone(880, 60, 100, TONE_PULSE1)".to_string());
        // tone(262, 60, 100, TONE_PULSE1);
    }
}
