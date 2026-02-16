#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use device_envoy::audio_player::{AtEnd, VOICE_22050_HZ, Volume, audio_player, pcm_clip};
use device_envoy::ir::{IrKepler, IrKeplerStatic, KeplerButton};
use embassy_executor::Spawner;
use panic_probe as _;

audio_player! {
    IrVoicePlayer {
        data_pin: PIN_8,
        bit_clock_pin: PIN_9,
        word_select_pin: PIN_10,
        sample_rate_hz: VOICE_22050_HZ,
        pio: PIO1,
        max_volume: Volume::spinal_tap(11),
    }
}

pcm_clip! {
    Digit0 {
        source_sample_rate_hz: VOICE_22050_HZ,
        target_sample_rate_hz: IrVoicePlayer::SAMPLE_RATE_HZ,
        file: "../data/audio/digits/0_22050.s16",
    }
}

pcm_clip! {
    Digit1 {
        source_sample_rate_hz: VOICE_22050_HZ,
        target_sample_rate_hz: IrVoicePlayer::SAMPLE_RATE_HZ,
        file: "../data/audio/digits/1_22050.s16",
    }
}

pcm_clip! {
    Digit2 {
        source_sample_rate_hz: VOICE_22050_HZ,
        target_sample_rate_hz: IrVoicePlayer::SAMPLE_RATE_HZ,
        file: "../data/audio/digits/2_22050.s16",
    }
}

pcm_clip! {
    Digit3 {
        source_sample_rate_hz: VOICE_22050_HZ,
        target_sample_rate_hz: IrVoicePlayer::SAMPLE_RATE_HZ,
        file: "../data/audio/digits/3_22050.s16",
    }
}

pcm_clip! {
    Digit4 {
        source_sample_rate_hz: VOICE_22050_HZ,
        target_sample_rate_hz: IrVoicePlayer::SAMPLE_RATE_HZ,
        file: "../data/audio/digits/4_22050.s16",
    }
}

pcm_clip! {
    Digit5 {
        source_sample_rate_hz: VOICE_22050_HZ,
        target_sample_rate_hz: IrVoicePlayer::SAMPLE_RATE_HZ,
        file: "../data/audio/digits/5_22050.s16",
    }
}

pcm_clip! {
    Digit6 {
        source_sample_rate_hz: VOICE_22050_HZ,
        target_sample_rate_hz: IrVoicePlayer::SAMPLE_RATE_HZ,
        file: "../data/audio/digits/6_22050.s16",
    }
}

pcm_clip! {
    Digit7 {
        source_sample_rate_hz: VOICE_22050_HZ,
        target_sample_rate_hz: IrVoicePlayer::SAMPLE_RATE_HZ,
        file: "../data/audio/digits/7_22050.s16",
    }
}

pcm_clip! {
    Digit8 {
        source_sample_rate_hz: VOICE_22050_HZ,
        target_sample_rate_hz: IrVoicePlayer::SAMPLE_RATE_HZ,
        file: "../data/audio/digits/8_22050.s16",
    }
}

pcm_clip! {
    Digit9 {
        source_sample_rate_hz: VOICE_22050_HZ,
        target_sample_rate_hz: IrVoicePlayer::SAMPLE_RATE_HZ,
        file: "../data/audio/digits/9_22050.s16",
    }
}

// todo00 `IrVoicePlayerAudioClipSource` us a trait and name is long.
const DIGITS: [&'static IrVoicePlayerAudioClipSource; 10] = [
    &Digit0::adpcm256_clip_from(Digit0::pcm_clip()),
    &Digit1::pcm_clip(),
    &Digit2::pcm_clip(),
    &Digit3::pcm_clip(),
    &Digit4::pcm_clip(),
    &Digit5::pcm_clip(),
    &Digit6::pcm_clip(),
    &Digit7::pcm_clip(),
    &Digit8::pcm_clip(),
    &Digit9::pcm_clip(),
];

const SPINAL_TAP_MIN: u8 = 0;
const SPINAL_TAP_MAX: u8 = 11;
const SPINAL_TAP_INIT: u8 = 5;
const SPINAL_TAP_DELTA_UP: i8 = 1;
const SPINAL_TAP_DELTA_DOWN: i8 = -1;

fn apply_spinal_tap_delta(spinal_tap_level: u8, delta: i8) -> u8 {
    assert!(delta == SPINAL_TAP_DELTA_DOWN || delta == SPINAL_TAP_DELTA_UP);
    assert!(spinal_tap_level <= SPINAL_TAP_MAX);
    if delta == SPINAL_TAP_DELTA_UP {
        if spinal_tap_level < SPINAL_TAP_MAX {
            spinal_tap_level + 1
        } else {
            SPINAL_TAP_MAX
        }
    } else if spinal_tap_level > SPINAL_TAP_MIN {
        spinal_tap_level - 1
    } else {
        SPINAL_TAP_MIN
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(Default::default());

    // Same IR wiring pattern as device-envoy/examples/conway.rs:
    // IR receiver data pin on GPIO15.
    static IR_KEPLER_STATIC: IrKeplerStatic = IrKepler::new_static();
    let ir_kepler = IrKepler::new(&IR_KEPLER_STATIC, p.PIN_15, p.PIO0, spawner).unwrap();

    let ir_voice_player =
        IrVoicePlayer::new(p.PIN_8, p.PIN_9, p.PIN_10, p.PIO1, p.DMA_CH0, spawner).unwrap();
    let mut spinal_tap_level = SPINAL_TAP_INIT;
    let mut last_non_mute_spinal_tap_level = if SPINAL_TAP_INIT == SPINAL_TAP_MIN {
        SPINAL_TAP_MAX
    } else {
        SPINAL_TAP_INIT
    };
    ir_voice_player.set_volume(Volume::spinal_tap(spinal_tap_level));
    info!("Audio startup check: speaking 0");
    ir_voice_player.play([DIGITS[0]], AtEnd::Stop);

    loop {
        let button = ir_kepler.wait_for_press().await;
        match button {
            KeplerButton::Num(number) => {
                info!("Playing digit {}", number);
                if let Some(digit_audio_clip) = DIGITS.get(number as usize).copied() {
                    ir_voice_player.play([digit_audio_clip], AtEnd::Stop);
                }
            }
            KeplerButton::Mute => {
                if spinal_tap_level == SPINAL_TAP_MIN {
                    spinal_tap_level = last_non_mute_spinal_tap_level;
                } else {
                    last_non_mute_spinal_tap_level = spinal_tap_level;
                    spinal_tap_level = SPINAL_TAP_MIN;
                }
                ir_voice_player.set_volume(Volume::spinal_tap(spinal_tap_level));
                info!("Volume set to {}/11", spinal_tap_level);
            }
            KeplerButton::Minus => {
                spinal_tap_level = apply_spinal_tap_delta(spinal_tap_level, SPINAL_TAP_DELTA_DOWN);
                if spinal_tap_level > SPINAL_TAP_MIN {
                    last_non_mute_spinal_tap_level = spinal_tap_level;
                }
                ir_voice_player.set_volume(Volume::spinal_tap(spinal_tap_level));
                info!("Volume set to {}/11", spinal_tap_level);
            }
            KeplerButton::Plus => {
                spinal_tap_level = apply_spinal_tap_delta(spinal_tap_level, SPINAL_TAP_DELTA_UP);
                if spinal_tap_level > SPINAL_TAP_MIN {
                    last_non_mute_spinal_tap_level = spinal_tap_level;
                }
                ir_voice_player.set_volume(Volume::spinal_tap(spinal_tap_level));
                info!("Volume set to {}/11", spinal_tap_level);
            }
            _ => {
                info!("IR mapped non-number button");
            }
        }
    }
}
