# Mimik

Mimik is a programmable real-time voice transformation engine written in Rust.

It captures audio from your microphone, processes it through a configurable voice pipeline, and exposes the transformed audio as a virtual microphone that can be used by applications such as Discord, Zoom, Google Meet, OBS, games, and any software capable of receiving microphone input.

The long-term goal of Mimik is to become the open platform for real-time voice processing. Instead of shipping with only a handful of built-in effects, Mimik is designed around downloadable voice packs, allowing anyone to create, share, install, and use new voices from the command line.

The project is built with performance and extensibility as first-class goals. Audio processing is designed to run with low latency, minimal allocations, and predictable performance while remaining modular enough to support both traditional DSP effects and future AI-powered voice conversion models.

## Vision

Mimik separates the audio engine from the voices themselves.

A voice is simply another package that can be installed, removed, updated, or shared without modifying the core application.

The same processing engine should be capable of running a simple robot effect, a cartoon voice, a cinematic radio filter, or an advanced AI voice conversion model through a common interface.

## Features

Real-time microphone processing

Low-latency audio pipeline

Programmable DSP effect graph

Downloadable voice packs

Local voice pack development

Cross-platform architecture

Extensible plugin-like voice system

Future AI voice model support

## Planned Commands

```bash
mimik devices

mimik live

mimik live robot

mimik list

mimik search robot

mimik install robot

mimik remove robot

mimik info robot

mimik status

mimik stop
```

## Architecture

```
Physical Microphone
        │
        ▼
   Audio Capture
        │
        ▼
 Processing Pipeline
        │
        ▼
 Voice Engine
        │
        ▼
 Virtual Microphone
        │
        ▼
 Discord • Zoom • OBS • Games
```

## Workspace

```
crates/
    mimik-cli
    mimik-core
    mimik-audio
    mimik-dsp
    mimik-pack
    mimik-registry
```

Each crate has a single responsibility.

`mimik-cli` contains the command-line interface.

`mimik-core` contains shared types and abstractions.

`mimik-audio` manages audio devices, streams, buffering, and platform integration.

`mimik-dsp` implements the real-time signal processing pipeline and voice effects.

`mimik-pack` handles voice pack installation, validation, and packaging.

`mimik-registry` communicates with remote registries and manages downloadable voices.

## Roadmap

The first milestone is a stable real-time audio pipeline capable of routing microphone audio to a virtual microphone with minimal latency.

The second milestone introduces a modular DSP engine with configurable effect chains.

The third milestone adds installable voice packs that describe complete voices through configuration rather than hardcoded implementations.

The fourth milestone introduces an online registry for discovering and downloading community-created voices.

The final milestone adds AI-powered voice conversion while preserving the same command-line workflow and voice package format.

## Philosophy

Mimik is built around a simple idea.

The application should not know how to be a robot, a cartoon character, or a narrator.

It should only know how to process audio.

Everything else should be another downloadable voice.
****