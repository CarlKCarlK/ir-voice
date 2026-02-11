# ir-voice

A Raspberry Pi Pico 1/2 app scaffold for IR + spoken number playback with `device-envoy`.

## Project status

This project is scaffolded and ready to add runtime logic.

Audio player wiring defaults in `src/main.rs`:

- DIN -> GP8
- BCLK -> GP9
- LRC -> GP10

## Build and run

Pico 1:

```bash
cargo ir-voice
```

Pico 2:

```bash
cargo ir-voice-2
```

## Generate number audio clips (0-9)

The script `scripts/generate_digit_pcm.py` generates raw PCM files compatible with `audio_clip!`.

Output format:

- mono
- signed 16-bit little-endian (`s16le`)
- 22050 Hz
- one file per digit in `data/audio/digits/`

Prerequisites:

- `uv` installed
- a system TTS backend for `pyttsx3` (for example, `espeak-ng` on Linux)
- `ffmpeg` on `PATH`

Run:

```bash
uv run --with pyttsx3 python scripts/generate_digit_pcm.py
```

List available voices:

```bash
uv run --with pyttsx3 python scripts/generate_digit_pcm.py --list-voices
```

Optional voice and sample rate:

```bash
uv run --with pyttsx3 python scripts/generate_digit_pcm.py --voice english --sample-rate 22050
```
