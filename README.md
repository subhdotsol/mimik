# Mimik

Mimik is a real-time programmable voice changer for macOS written in Rust.

It captures audio from your microphone, processes it through a configurable DSP pipeline, and outputs the transformed audio to a virtual microphone that any application can receive — Discord, Zoom, OBS, games, or anything that reads microphone input.

Voices are described entirely in TOML. Installing a new voice is a single command. No GUI, no driver reinstalls, no application restarts.

## Prerequisites

**rubberband** is required for pitch and formant shifting. Install it before running `cargo install mimik`.

```bash
brew install rubberband
```

**BlackHole 2ch** is required as the virtual audio device that mimik writes to. Download it from [existential.audio/blackhole](https://existential.audio/blackhole/).

After installing BlackHole, open **Audio MIDI Setup**, create a Multi-Output Device that combines your speakers and BlackHole 2ch, and set that as your system output. Then set BlackHole 2ch as the microphone input in Discord, Zoom, or whichever application you want to receive the processed audio.

## Installation

```bash
cargo install mimik
```

## Commands

### List available audio devices

```bash
mimik devices
```

Prints all input and output devices recognised by the system. Use this to confirm BlackHole 2ch appears as an output device.

### Start live voice processing

```bash
mimik live
mimik live <voice>
```

Starts the audio pipeline. Without a voice argument, audio is passed through unmodified. With a voice, the full DSP chain for that voice is applied in real time.

```bash
mimik live tom
mimik live soft-feminine
```

Press Ctrl+C to stop.

### Check pipeline status

```bash
mimik status
```

Shows whether the pipeline is running, which voice is active, and the process ID.

### Stop the pipeline

```bash
mimik stop
```

Sends a termination signal to the running pipeline process.

### List installed voices

```bash
mimik list
```

Prints all voices installed under the voices directory.

### Inspect a voice

```bash
mimik info <voice>
```

Prints the manifest and full effects chain for a voice.

```bash
mimik info soft-feminine
```

### Install a voice pack

```bash
mimik install <path>
```

Copies a local voice pack directory into the voices directory after validating its manifest.

```bash
mimik install ./my-voice
```

### Remove a voice

```bash
mimik remove <voice>
```

Deletes an installed voice. Built-in voices cannot be removed.

## Voice Packs

A voice pack is a directory containing two files.

**manifest.toml**

```toml
id = "my-voice"
name = "My Voice"
version = "0.1.0"
engine = "dsp"
description = "A custom voice"
```

**preset.toml**

```toml
[[effects]]
type = "noise_gate"
threshold_db = -48.0

[[effects]]
type = "high_pass"
frequency_hz = 100.0

[[effects]]
type = "pitch_shift"
semitones = 4.0

[[effects]]
type = "formant_shift"
ratio = 1.2

[[effects]]
type = "reverb"
room_size = 0.2
mix = 0.1
decay_ms = 300.0

[[effects]]
type = "limiter"
ceiling_db = -1.0
```

### Available effects

| Effect | Parameters |
|---|---|
| `noise_gate` | `threshold_db` |
| `high_pass` | `frequency_hz` |
| `pitch_shift` | `semitones` |
| `formant_shift` | `ratio` |
| `equalizer` | `frequency_hz`, `gain_db`, `q` |
| `compressor` | `threshold_db`, `ratio`, `attack_ms`, `release_ms` |
| `limiter` | `ceiling_db` |
| `vibrato` | `rate_hz`, `depth_cents`, `mix` |
| `de_esser` | `frequency_hz`, `threshold_db` |
| `chorus` | `rate_hz`, `depth_ms`, `mix` |
| `reverb` | `room_size`, `mix`, `decay_ms` |

Effects before `pitch_shift` in the list run before pitch processing. Effects after it run on the shifted audio. `formant_shift` always applies to the pitch shifter regardless of its position in the list.

Install your pack with:

```bash
mimik install ./my-voice
mimik live my-voice
```

## Architecture

```
Physical Microphone
        |
        v
   Audio Capture (cpal)
        |
        v
   Mono Mix-Down
        |
        v
   Pre-Pitch Effects
        |
        v
   Pitch + Formant Shift (RubberBand)
        |
        v
   Post-Pitch Effects
        |
        v
   BlackHole 2ch (virtual microphone)
        |
        v
Discord / Zoom / OBS / Games
```

## Workspace

```
crates/
    mimik            command-line interface and entry point
    mimik-core       shared types, voice manifest schema, preset definitions
    mimik-audio      audio device management, stream setup, processing pipeline
    mimik-dsp        DSP effects: pitch shifting, EQ, compressor, reverb, chorus
    mimik-pack       voice pack packaging and validation (in development)
    mimik-registry   remote registry client for discovering voices (in development)
```

## Roadmap

Real-time DSP pipeline with installable voice packs is complete.

Next: online registry for discovering and downloading community voices (`mimik search`, `mimik publish`).

After that: AI-powered voice conversion running through the same voice pack interface.

## License

MIT

