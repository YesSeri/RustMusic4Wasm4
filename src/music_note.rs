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
}
// impl From<MusicNote> for String {
//     fn from(m: MusicNote) -> Self {
//         let MusicNote { note, octave } = m;
//         // "tone(440, 60, 100, TONE_PULSE1)".to_string()
//     }
// }

#[cfg(test)]
mod tests {
    use crate::{Alter, MusicNote};
    #[test]
    fn it_works() {
        let m = MusicNote::new("A".to_string(), 4, Alter::Natural);
        // let s: String = m.into();
        // assert_eq!(s, "tone(440, 60, 100, TONE_PULSE1)".to_string());
        // tone(262, 60, 100, TONE_PULSE1);
    }
}
