use roxmltree::*;

use crate::get_music_note;
use crate::music_info::MusicInfo;

fn get_elements_by_tag_name<'a>(node: &'a Node, tag_name: &str) -> Vec<Node<'a, 'a>> {
    node.descendants()
        .filter(|n| n.has_tag_name(tag_name))
        .collect()
}

pub fn get_element_by_tag_name<'a>(node: &'a Node, tag_name: &str) -> Option<Node<'a, 'a>> {
    Some(node.descendants().find(|n| n.has_tag_name(tag_name))?)
}

pub fn get_measures(part: Node) -> Vec<Vec<MusicInfo>> {
    let measures = get_elements_by_tag_name(&part, "measure");
    let bars: Vec<Vec<MusicInfo>> = measures
        .iter()
        .map(|m| {
            return get_bar(m);
        })
        .collect();
    bars
}
fn get_bar(measure: &Node) -> Vec<MusicInfo> {
    let notes = get_elements_by_tag_name(&measure, "note");
    let music_bar = notes
        .into_iter()
        .map(|note| {
            return MusicInfo {
                note: get_music_note(&note),
                duration: get_text_from_tag(&note, "duration")
                    .unwrap()
                    .parse()
                    .unwrap(),
            };
        })
        .collect();
    music_bar
}

pub fn get_text_from_tag(node: &Node, tag_name: &str) -> Option<String> {
    Some(
        get_element_by_tag_name(node, tag_name)?
            .text()
            .unwrap()
            .to_string(),
    )
}
