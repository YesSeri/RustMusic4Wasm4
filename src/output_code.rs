use crate::music_info::MusicInfo;

// Bars of beats
pub fn output_code(music: Vec<Vec<MusicInfo>>, voice: u8) {
    let mut code: String = String::new();
    let instrument = match voice {
        0 => "TONE_PULSE1",
        1 => "TONE_PULSE2",
        2 => "TONE_TRIANGLE",
        3 => "TONE_NOISE",
        4 => "TONE_CUSTOM",
        5 => "TONE_CUSTOM",
        _ => unreachable!(),
    };
    for (bar_idx, bar) in music.iter().enumerate() {
        code += &format!("\nfn play_bar_{}_voice_{}(subbeat: u8){{\n", bar_idx, voice);
        code += "\tmatch subbeat {\n";
        let mut curr_beat = 0;
        for note in bar {
            code += &format!("\t\t{} => {{", curr_beat);
            curr_beat += note.duration;

            match &note.note {
                Some(n) => {
                    let hz: u32 = n.pitch.into();
                    let string: String = n.pitch.into();
                    code += &format!(
                        "\n\t\t\ttone({}, 30, 100, {}) // {}\n\t\t}}\n",
                        hz, instrument, string
                    )
                    .to_string();
                }
                None => {}
            }
        }

        code += "\t}\n}\n"
    }
    println!("{}", code);
}
