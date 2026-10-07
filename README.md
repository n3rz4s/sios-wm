# SIOS-WM

**SIOS-WM** is an experimental Wayland compositor written in Rust.

It is being developed as a core component of the **SIOS desktop environment**, with a focus on low-level Wayland, DRM/KMS, GPU rendering, configuration APIs, and compositor architecture.

> [!WARNING]
> **SIOS-WM is experimental software and is currently under active development.**
>
> The project is not production-ready. The architecture, APIs, configuration system, rendering pipeline, and internal implementation may change at any time.

---

## Overview

SIOS-WM is a custom Wayland compositor built with Rust and [Smithay](https://github.com/Smithay/smithay).

The project is designed to provide the compositor layer for the SIOS desktop environment while allowing low-level control over rendering, outputs, windows, input devices, configuration, and shaders.

The project is currently developed primarily for Linux systems using DRM/KMS.

---

## Features

Current development includes:

- Wayland compositor
- DRM/KMS backend
- GBM
- EGL
- OpenGL ES rendering
- libseat session management
- udev device management
- libinput input handling
- XDG Shell
- XDG Decorations
- Layer Shell
- Viewporter
- Output and monitor management
- Window management
- Cursor rendering
- Custom render elements
- Custom OpenGL shaders
- Window rules
- Runtime configuration
- Dynamic configuration API
- Shader API
- Experimental rendering pipeline

---

## Architecture

The compositor is divided into several major subsystems.

```text
                         Wayland Clients
                               │
                               ▼
                        Wayland Protocol
                               │
                               ▼
                         Compositor State
                               │
              ┌────────────────┼────────────────┐
              │                │                │
              ▼                ▼                ▼
           Backend          Desktop        Configuration
              │                │                │
        ┌─────┼─────┐          │          ┌─────┼─────┐
        │     │     │          │          │     │     │
       DRM   GBM   Input       │       Monitor Window Shader
        │     │     │           │
        └─────┼─────┘           ▼
              │              Renderer
              ▼                 │
        GPU / Display     ┌────┼────┬──────┐
                          │    │    │      │
                       Surface Cursor Border Shaders
```

The architecture is actively evolving as new compositor functionality is implemented.

---

## APIs

SIOS-WM currently uses separate API crates for different parts of the compositor:

### `sios-wm-api`

Provides the public API used by the compositor configuration system.

### `sios-wm-shader-api`

Provides the API used for custom shader functionality.

Both APIs are currently considered **unstable** and may change as the compositor architecture evolves.

---

## Configuration

SIOS-WM includes an experimental configuration system.

Configuration can be compiled and loaded dynamically, allowing compositor behavior such as window rules, monitor settings, and shader configuration to be controlled externally.

The configuration system is still under active development.

---

## Rendering

Rendering is built around Smithay's renderer infrastructure and OpenGL ES.

The rendering subsystem currently includes:

- Wayland surface rendering
- custom render elements
- cursor rendering
- borders
- shader-based effects
- damage handling
- GPU resource management
- output rendering

Rendering performance and damage tracking are currently being optimized.

---

## Development Status

SIOS-WM is currently **WIP / Experimental**.

The compositor is already capable of:

- initializing the graphics stack;
- initializing DRM/KMS;
- creating an EGL/OpenGL ES context;
- creating a Wayland socket;
- accepting Wayland clients;
- handling Wayland surfaces;
- managing windows;
- rendering compositor elements;
- handling input devices;
- applying compositor configuration.

However, many parts of the implementation are still being developed and refactored.

### Expect

- breaking API changes;
- architectural changes;
- incomplete features;
- rendering bugs;
- temporary code;
- platform-specific behavior;
- unstable configuration formats;
- changes to build requirements.

---

## Building

At the current stage, SIOS-WM is primarily a development project.

The repository may contain development-specific paths and dependencies and is **not yet guaranteed to build on another system without modification**.

A portable build and installation system is planned as the project matures.

### Requirements

The project currently targets Linux and uses technologies including:

- Rust
- Wayland
- Smithay
- DRM/KMS
- GBM
- EGL
- OpenGL ES
- libseat
- libinput
- udev

Exact dependency versions may change during development.

---

## Project Structure

The project is organized around several major components:

```text
sios-wm/
├── src/
│   ├── backend/
│   ├── config/
│   ├── desktop/
│   │   └── render/
│   ├── init/
│   └── wayland/
│
├── sios-wm-api/
│
├── sios-wm-shader-api/
│
├── Cargo.toml
├── Cargo.lock
└── README.md
```

The structure may change during future refactoring.

---

## Goals

The long-term goal is to develop SIOS-WM into a complete Wayland compositor for the SIOS desktop environment.

Planned areas include:

- improved rendering performance;
- proper damage tracking;
- optimized redraws;
- improved multi-monitor support;
- improved window management;
- stable configuration API;
- stable shader API;
- cleaner compositor state management;
- improved plugin/configuration support;
- portable build system;
- installation and packaging;
- better documentation.

---

## Author / Contact

**Developer:** n3rz4

- Telegram Channel: [@n3rz4_channel](https://t.me/n3rz4_channel)
- Telegram: [@n3rz4_bloody_phoenix](https://t.me/n3rz4_bloody_phoenix)

---

## License

License information will be added as the project develops.

---

## Disclaimer

SIOS-WM is experimental software.

**Do not use it as your primary desktop compositor.**

The project is being developed for experimentation, research, learning, and the development of the SIOS desktop environment.

The APIs, architecture, configuration format, and internal implementation are not considered stable.