# RustMusic4Wasm4

Music4Wasm4 lets you create your music in any program and then export it to wasm4. This is still in alpha stage.

You can write your music in MuseScore, or any other program that exports to musicxml, non-compressed and run it and it will output rust wasm4 code in the console that you can copy paste.

## Usage

Download it and run it with `cargo run`. There is some example music I have written. `cargo run ./assets/xml-cello-violin-test.musicxml`. Test with them or use your own music.

## Beats

The subbeat you have to supply depends on the fastest note you have in your music. If your music is in 3/4, three beats per bar, and you use semiquavers as the fastest note (4 notes per beat) then you will have 3 \* 4 = 12 subbeats per bar, from 0 up to 11.

## Creating subbeat and choosing music.

This will take some simple math. If your song is 4 bars long, and there is 8 subbeats per bar that gives us a total of 4 \* 8 = 32 subbeats total. That means we need to restart the song from the start every 32 subbeats. The function might look like this:

```rust
fn music_player(counter: &mut u8) {
    if *counter == 32 {
        *counter = 0;
    }

    let bar = *counter / 4;
    let subbeat = *counter % 4;
    match bar {
        0 => {
            play_bar_0_voice_0(subbeat);
            play_bar_0_voice_1(subbeat);
        }
        1 => {
            play_bar_1_voice_0(subbeat);
            play_bar_1_voice_1(subbeat);
        }
        2 => {
            play_bar_2_voice_0(subbeat);
            play_bar_2_voice_1(subbeat);
        }
        3 => {
            play_bar_3_voice_0(subbeat);
            play_bar_3_voice_1(subbeat);
        }
        _ => unreachable!(),
    }
}
```

## Beat changes

You can't change beat from 3/4 to 4/4 or 6/8 or anything at all. The rhythm has to be the same all the way through.

This of course needs to be fixed.

## Ideas

I am sure this code generating thing could be solved a lot better with macros. It might also be possible to make this into a library where you just call `music4wasm4!("./path/to/music.musicxml")` and it gets included when built. If somebody is comfortable with macros, please have a look at that!
