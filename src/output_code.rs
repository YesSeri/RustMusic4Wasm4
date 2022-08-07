use crate::music_info::MusicInfo;

// Bars of beats
pub fn output_code(music: Vec<Vec<MusicInfo>>, voice: u8) {
    let mut code: String = String::new();
    for (bar_idx, bar) in music.iter().enumerate() {
        code += &format!("\nfn play_bar_{}(subbeat: u8){{\n", bar_idx);
        code += "\tmatch subbeat {\n";
        let mut curr_beat = 0;
        for note in bar {
            code += &format!("\t\t{} => {{", curr_beat);
            curr_beat += note.duration;

            match &note.note {
                Some(n) => {
                    let string: String = n.pitch.into();

                    code += &format!("\n\t\t\ttone({}, 30, 100, TONE_PULSE1)\n\t\t}}\n", string)
                        .to_string();
                }
                None => {}
            }
        }

        code += "\t}\n}\n"
    }
    println!("{}", code);
}
