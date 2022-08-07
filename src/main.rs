#![allow(unused_variables)]
#![allow(dead_code)]
mod alter;
mod music_info;
mod music_note;
mod note;
mod output_code;
mod parse_xml;
mod to_wasm4;
use alter::Alter;
use music_note::MusicNote;
use parse_xml::{get_element_by_tag_name, get_measures, get_text_from_tag};
use roxmltree::*;
use std::env;
use std::fs::File;
use std::io::Read;

// <note>
//   <pitch>
//     <step>D</step>
//     <octave>3</octave>
//   </pitch>
//   <duration>2</duration>
// </note>
fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];
    // let data = get_file_data("./assets/xml-one-bar-two-voices.musicxml").unwrap();
    let data = get_file_data(file_name).unwrap();
    let doc = Document::parse(&data).unwrap();
    let parts: Vec<Node> = doc
        .descendants()
        .filter(|n| n.has_tag_name("part"))
        .collect();

    let subbeats = get_subbeats(&parts[0]);

    for (voice_idx, part) in parts.into_iter().enumerate() {
        let measures = get_measures(part);
        output_code::output_code(measures, voice_idx as u8);
    }
}

fn get_file_data(file_name: &str) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
fn get_music_note(note: &Node) -> Option<MusicNote> {
    let step = get_text_from_tag(note, "step")?;
    let octave = get_text_from_tag(note, "octave")?.parse().unwrap();
    let alter: Alter = get_text_from_tag(note, "alter").into();
    Some(MusicNote::new(step, octave, alter))
}

fn get_subbeats(node: &Node) -> usize {
    let el = get_element_by_tag_name(node, "measure").unwrap();
    let el = get_element_by_tag_name(&el, "attributes").unwrap();
    let divisions = get_text_from_tag(&el, "divisions").unwrap();
    let el = get_element_by_tag_name(&el, "time").unwrap();
    let beats = get_text_from_tag(&el, "beats").unwrap();
    beats.parse::<usize>().unwrap() * divisions.parse::<usize>().unwrap()
}
