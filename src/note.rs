use crate::alter::Alter;

/// C4			261.63 	131.87
/// C#4 	 	277.18 	124.47
/// D4			293.66 	117.48
/// D#4 	 	311.13 	110.89
/// E4			329.63 	104.66
/// F4			349.23 	98.79
/// F#4 	 	369.99 	93.24
/// G4			392.00 	88.01
/// G#4		  	415.30 	83.07
/// A4			440.00 	78.41
/// A#4 	 	466.16 	74.01
/// B4			493.88 	69.85
/// C5			523.25 	65.93
#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum Note {
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
    A,
    Bb,
    B,
}

impl Note {
    pub fn get_note(step: String, alter: Alter) -> Self {
        match step.as_str() {
            "A" => match alter {
                Alter::Lowered => Note::Ab,
                Alter::Natural => Note::A,
                Alter::Sharp => Note::Bb,
            },
            "B" => match alter {
                Alter::Lowered => Note::Bb,
                Alter::Natural => Note::B,
                Alter::Sharp => unreachable!(),
            },
            "C" => match alter {
                Alter::Lowered => unreachable!(),
                Alter::Natural => Note::C,
                Alter::Sharp => Note::Db,
            },
            "D" => match alter {
                Alter::Lowered => Note::Db,
                Alter::Natural => Note::D,
                Alter::Sharp => Note::Eb,
            },
            "E" => match alter {
                Alter::Lowered => Note::Eb,
                Alter::Natural => Note::E,
                Alter::Sharp => unreachable!(),
            },
            "F" => match alter {
                Alter::Lowered => unreachable!(),
                Alter::Natural => Note::F,
                Alter::Sharp => Note::Gb,
            },
            "G" => match alter {
                Alter::Lowered => Note::Gb,
                Alter::Natural => Note::G,
                Alter::Sharp => Note::Ab,
            },
            _ => unreachable!(),
        }
    }
}
// pub enum Note {
//     C = 262,
//     Db = 277,
//     D = 294,
//     Eb = 311,
//     E = 330,
//     F = 349,
//     Gb = 370,
//     G = 392,
//     Ab = 415,
//     A = 440,
//     Bb = 466,
//     B = 494,
// }
impl From<Note> for u32 {
    fn from(hz: Note) -> Self {
        match hz {
            Note::C => 262,
            Note::Db => 277,
            Note::D => 294,
            Note::Eb => 311,
            Note::E => 330,
            Note::F => 349,
            Note::Gb => 370,
            Note::G => 392,
            Note::Ab => 415,
            Note::A => 440,
            Note::Bb => 466,
            Note::B => 494,
        }
    }
}
impl From<Note> for String {
    #[rustfmt::skip]
    fn from(hz: Note) -> Self {
        match hz {
            Note::C 	=> "C".to_string(),
            Note::Db 	=> "Db".to_string(),
            Note::D 	=> "D".to_string(),
            Note::Eb 	=> "Eb".to_string(),
            Note::E 	=> "E".to_string(),
            Note::F 	=> "F".to_string(),
            Note::Gb 	=> "Gb".to_string(),
            Note::G 	=> "G".to_string(),
            Note::Ab 	=> "Ab".to_string(),
            Note::A 	=> "A".to_string(),
            Note::Bb 	=> "Bb".to_string(),
            Note::B 	=> "B".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::note::Note;

    #[test]
    fn convert_note_to_number() {
        let note = Note::A;
        assert_eq!(440 as u32, note.into(),);
    }
}
