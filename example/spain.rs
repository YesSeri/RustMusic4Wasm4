// If you just run update_music() in your update function this music will play. Limiter is used to not have to high tempo in music.
use crate::wasm4::*;

static mut LIMITER: u8 = 0;
static mut COUNTER: u8 = 0;
pub fn update_music() {
    unsafe {
        LIMITER += 1;
        if LIMITER < 8 {
            return;
        }
        LIMITER = 0;
    }
    let c = unsafe {
        if COUNTER == 40 {
            COUNTER = 0;
        }
        COUNTER
    };

    let bar = c / 8;
    let subbeat = c % 8;

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
        4 => {
            play_bar_4_voice_0(subbeat);
            play_bar_4_voice_1(subbeat);
        }
        _ => (),
    }

    unsafe {
        COUNTER += 1;
    }
}

fn play_bar_0_voice_0(subbeat: u8) {
    match subbeat {
        0 => {
            tone(392, 30, 100, TONE_PULSE1) // G
        }
        2 => {
            tone(440, 30, 100, TONE_PULSE1) // A
        }
        4 => {
            tone(494, 30, 100, TONE_PULSE1) // B
        }
        _ => {}
    }
}

fn play_bar_1_voice_0(subbeat: u8) {
    match subbeat {
        0 => {
            tone(392, 30, 100, TONE_PULSE1) // G
        }
        2 => {
            tone(440, 30, 100, TONE_PULSE1) // A
        }
        4 => {
            tone(494, 30, 100, TONE_PULSE1) // B
        }
        _ => {}
    }
}

fn play_bar_2_voice_0(subbeat: u8) {
    match subbeat {
        0 => {
            tone(440, 30, 100, TONE_PULSE1) // A
        }
        3 => {
            tone(440, 30, 100, TONE_PULSE1) // A
        }
        4 => {
            tone(440, 30, 100, TONE_PULSE1) // A
        }
        6 => {
            tone(440, 30, 100, TONE_PULSE1) // A
        }
        _ => {}
    }
}

fn play_bar_3_voice_0(subbeat: u8) {
    match subbeat {
        0 => {
            tone(392, 30, 100, TONE_PULSE1) // G
        }
        4 => {
            tone(392, 30, 100, TONE_PULSE1) // G
        }
        _ => {}
    }
}

fn play_bar_4_voice_0(subbeat: u8) {
    match subbeat {
        0 => {}
        _ => {}
    }
}

fn play_bar_0_voice_1(subbeat: u8) {
    match subbeat {
        0 => {
            tone(196, 30, 100, TONE_PULSE2) // G
        }
        2 => {
            tone(147, 30, 100, TONE_PULSE2) // D
        }
        4 => {
            tone(196, 30, 100, TONE_PULSE2) // G
        }
        _ => {}
    }
}

fn play_bar_1_voice_1(subbeat: u8) {
    match subbeat {
        0 => {
            tone(196, 30, 100, TONE_PULSE2) // G
        }
        2 => {
            tone(147, 30, 100, TONE_PULSE2) // D
        }
        4 => {
            tone(196, 30, 100, TONE_PULSE2) // G
        }
        _ => {}
    }
}

fn play_bar_2_voice_1(subbeat: u8) {
    match subbeat {
        0 => {
            tone(262, 30, 100, TONE_PULSE2) // C
        }
        2 => {
            tone(131, 30, 100, TONE_PULSE2) // C
        }
        4 => {
            tone(147, 30, 100, TONE_PULSE2) // D
        }
        6 => {
            tone(147, 30, 100, TONE_PULSE2) // D
        }
        _ => {}
    }
}

fn play_bar_3_voice_1(subbeat: u8) {
    match subbeat {
        0 => {
            tone(196, 30, 100, TONE_PULSE2) // G
        }
        2 => {
            tone(147, 30, 100, TONE_PULSE2) // D
        }
        4 => {
            tone(98, 30, 100, TONE_PULSE2) // G
        }
        _ => {}
    }
}

fn play_bar_4_voice_1(subbeat: u8) {
    match subbeat {
        0 => {}
        _ => {}
    }
}
