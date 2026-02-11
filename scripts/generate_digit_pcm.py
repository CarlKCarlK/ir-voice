#!/usr/bin/env python3
"""Generate spoken digits 0..9 as raw PCM clips for device-envoy audio_clip!.

Output files are written to data/audio/digits/<digit>_<sample_rate>.s16.
"""

from __future__ import annotations

import argparse
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Generate 0..9 digit PCM clips via offline pyttsx3"
    )
    parser.add_argument(
        "--voice",
        default="",
        help="Optional voice name/id substring (for example: 'english', 'zira', 'david')",
    )
    parser.add_argument(
        "--rate-wpm",
        type=int,
        default=170,
        help="pyttsx3 speech rate in words per minute",
    )
    parser.add_argument(
        "--sample-rate",
        type=int,
        default=22_050,
        help="Output sample rate in Hz for generated .s16 files",
    )
    parser.add_argument(
        "--out-dir",
        default="data/audio/digits",
        help="Output directory for generated .s16 files",
    )
    parser.add_argument(
        "--list-voices",
        action="store_true",
        help="List available pyttsx3 voices and exit",
    )
    return parser.parse_args()


def ensure_ffmpeg() -> None:
    if shutil.which("ffmpeg") is None:
        raise RuntimeError("ffmpeg not found in PATH")


def pyttsx3_engine():
    try:
        import pyttsx3  # type: ignore[import-not-found]
    except ModuleNotFoundError as err:
        raise RuntimeError(
            "pyttsx3 is not installed. Run with uv: "
            "`uv run --with pyttsx3 python scripts/generate_digit_pcm.py`"
        ) from err

    try:
        engine = pyttsx3.init()
    except Exception as err:  # pragma: no cover - driver-specific runtime failure
        raise RuntimeError(
            "pyttsx3 initialization failed. Install a system TTS backend "
            "(for example, `espeak-ng` on Linux)."
        ) from err
    return engine


def voice_label(voice) -> str:
    voice_name = getattr(voice, "name", "")
    voice_id = getattr(voice, "id", "")
    return f"{voice_name} [{voice_id}]".strip()


def select_voice(engine, voice_query: str) -> None:
    if not voice_query:
        return

    voice_query = voice_query.lower()
    voices = engine.getProperty("voices") or []
    selected_voice = None

    for voice in voices:
        voice_name = getattr(voice, "name", "")
        voice_id = getattr(voice, "id", "")
        if voice_query in voice_name.lower() or voice_query in voice_id.lower():
            selected_voice = voice
            break

    if selected_voice is None:
        available_voice_labels = ", ".join(voice_label(voice) for voice in voices)
        raise RuntimeError(
            f"voice '{voice_query}' not found. Available voices: {available_voice_labels}"
        )

    engine.setProperty("voice", selected_voice.id)


def list_voices(engine) -> None:
    voices = engine.getProperty("voices") or []
    if not voices:
        print("No voices reported by pyttsx3.")
        return
    for voice in voices:
        print(voice_label(voice))


def synthesize_digit_wav(engine, digit_text: str, wav_path: Path) -> None:
    engine.save_to_file(digit_text, str(wav_path))
    engine.runAndWait()
    if not wav_path.exists() or wav_path.stat().st_size == 0:
        raise RuntimeError(f"pyttsx3 did not generate audio for '{digit_text}'")


def convert_wav_to_s16le(wav_path: Path, s16_path: Path, sample_rate: int) -> None:
    subprocess.run(
        [
            "ffmpeg",
            "-y",
            "-i",
            str(wav_path),
            "-vn",
            "-ac",
            "1",
            "-ar",
            str(sample_rate),
            "-f",
            "s16le",
            str(s16_path),
        ],
        check=True,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )


def main() -> int:
    args = parse_args()
    ensure_ffmpeg()

    sample_rate = args.sample_rate
    if sample_rate <= 0:
        print("sample-rate must be > 0", file=sys.stderr)
        return 2

    out_dir = Path(args.out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)

    engine = pyttsx3_engine()
    engine.setProperty("rate", args.rate_wpm)
    select_voice(engine, args.voice)

    if args.list_voices:
        list_voices(engine)
        engine.stop()
        return 0

    with tempfile.TemporaryDirectory(prefix="ir_voice_digits_") as temp_dir:
        temp_path = Path(temp_dir)

        for digit in range(10):
            digit_text = str(digit)
            wav_path = temp_path / f"{digit_text}.wav"
            s16_path = out_dir / f"{digit_text}_{sample_rate}.s16"

            synthesize_digit_wav(engine, digit_text, wav_path)
            convert_wav_to_s16le(wav_path, s16_path, sample_rate)
            print(f"generated {s16_path}")

    engine.stop()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
