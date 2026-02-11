#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use device_envoy::audio_player::{AtEnd, VOICE_22050_HZ, Volume, audio_clip, audio_player};
use device_envoy::ir::{IrKepler, IrKeplerStatic, KeplerButton};
use embassy_executor::Spawner;
use panic_probe as _;

audio_player! {
    IrVoicePlayer {
        din_pin: PIN_8,
        bclk_pin: PIN_9,
        lrc_pin: PIN_10,
        sample_rate_hz: VOICE_22050_HZ,
        max_volume: Volume::percent(20),
    }
}

audio_clip! {
    Digit0 {
        sample_rate_hz: VOICE_22050_HZ,
        file: "../data/audio/digits/0_22050.s16",
    }
}

audio_clip! {
    Digit1 {
        sample_rate_hz: VOICE_22050_HZ,
        file: "../data/audio/digits/1_22050.s16",
    }
}

audio_clip! {
    Digit2 {
        sample_rate_hz: VOICE_22050_HZ,
        file: "../data/audio/digits/2_22050.s16",
    }
}

audio_clip! {
    Digit3 {
        sample_rate_hz: VOICE_22050_HZ,
        file: "../data/audio/digits/3_22050.s16",
    }
}

audio_clip! {
    Digit4 {
        sample_rate_hz: VOICE_22050_HZ,
        file: "../data/audio/digits/4_22050.s16",
    }
}

audio_clip! {
    Digit5 {
        sample_rate_hz: VOICE_22050_HZ,
        file: "../data/audio/digits/5_22050.s16",
    }
}

audio_clip! {
    Digit6 {
        sample_rate_hz: VOICE_22050_HZ,
        file: "../data/audio/digits/6_22050.s16",
    }
}

audio_clip! {
    Digit7 {
        sample_rate_hz: VOICE_22050_HZ,
        file: "../data/audio/digits/7_22050.s16",
    }
}

audio_clip! {
    Digit8 {
        sample_rate_hz: VOICE_22050_HZ,
        file: "../data/audio/digits/8_22050.s16",
    }
}

audio_clip! {
    Digit9 {
        sample_rate_hz: VOICE_22050_HZ,
        file: "../data/audio/digits/9_22050.s16",
    }
}

static DIGIT0: Digit0::AudioClip = Digit0::audio_clip();
static DIGIT1: Digit1::AudioClip = Digit1::audio_clip();
static DIGIT2: Digit2::AudioClip = Digit2::audio_clip();
static DIGIT3: Digit3::AudioClip = Digit3::audio_clip();
static DIGIT4: Digit4::AudioClip = Digit4::audio_clip();
static DIGIT5: Digit5::AudioClip = Digit5::audio_clip();
static DIGIT6: Digit6::AudioClip = Digit6::audio_clip();
static DIGIT7: Digit7::AudioClip = Digit7::audio_clip();
static DIGIT8: Digit8::AudioClip = Digit8::audio_clip();
static DIGIT9: Digit9::AudioClip = Digit9::audio_clip();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(Default::default());

    // Same IR wiring pattern as device-envoy/examples/conway.rs:
    // IR receiver data pin on GPIO15.
    static IR_KEPLER_STATIC: IrKeplerStatic = IrKepler::new_static();
    let ir_kepler = IrKepler::new(&IR_KEPLER_STATIC, p.PIN_15, p.PIO0, spawner).unwrap();

    let ir_voice_player =
        IrVoicePlayer::new(p.PIN_8, p.PIN_9, p.PIN_10, p.PIO1, p.DMA_CH0, spawner).unwrap();

    info!("Audio startup check: speaking 0");
    ir_voice_player.play([&DIGIT0], AtEnd::Stop);

    loop {
        let button = ir_kepler.wait_for_press().await;

        match button {
            KeplerButton::Num(number) => {
                info!("IR mapped number: {}", number);
                match number {
                    0 => {
                        info!("Playing digit 0");
                        ir_voice_player.play([&DIGIT0], AtEnd::Stop);
                    }
                    1 => {
                        info!("Playing digit 1");
                        ir_voice_player.play([&DIGIT1], AtEnd::Stop);
                    }
                    2 => {
                        info!("Playing digit 2");
                        ir_voice_player.play([&DIGIT2], AtEnd::Stop);
                    }
                    3 => {
                        info!("Playing digit 3");
                        ir_voice_player.play([&DIGIT3], AtEnd::Stop);
                    }
                    4 => {
                        info!("Playing digit 4");
                        ir_voice_player.play([&DIGIT4], AtEnd::Stop);
                    }
                    5 => {
                        info!("Playing digit 5");
                        ir_voice_player.play([&DIGIT5], AtEnd::Stop);
                    }
                    6 => {
                        info!("Playing digit 6");
                        ir_voice_player.play([&DIGIT6], AtEnd::Stop);
                    }
                    7 => {
                        info!("Playing digit 7");
                        ir_voice_player.play([&DIGIT7], AtEnd::Stop);
                    }
                    8 => {
                        info!("Playing digit 8");
                        ir_voice_player.play([&DIGIT8], AtEnd::Stop);
                    }
                    9 => {
                        info!("Playing digit 9");
                        ir_voice_player.play([&DIGIT9], AtEnd::Stop);
                    }
                    _ => {}
                }
            }
            _ => {
                info!("IR mapped non-number button");
            }
        }
    }
}
