# PawnGFX

A minimal software renderer written in Rust for learning and experimentation with 3D graphics fundamentals.
The aim is to develop a fast and optimized light weight CPU based software renderer to view .obj and gltf models.

## Overview

BareGFX is a barebone graphics renderer that implements core 3D graphics concepts including:
- Software-based rasterization
- Camera system with perspective projection
- 3D transformations (rotation, translation, scaling)
- Wireframe rendering of 3D objects
- User input handling for object and camera control

## Features

- **3D Math Library**: Vector3, Vector4, and Matrix4 implementations with essential operations
- **Camera System**: Perspective camera with smooth movement and rotation
- **Wireframe Rendering**: Line-based drawing using Bresenham's line algorithm
- **Interactive Controls**: Keyboard input for camera movement and object rotation
- **Depth Management**: Depth buffer for proper rendering

## Building & Running

### Prerequisites
- Rust 1.70+

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run --release
```

## Controls

- **W/A/S/D**: Move camera forward/left/backward/right
- **Arrow Keys**: Rotate cube (↑/↓ for X-axis, ←/→ for Y-axis)

## Project Structure

- `src/main.rs` - Entry point and event loop
- `src/camera.rs` - Camera implementation
- `src/draw.rs` - Rendering functions
- `src/input.rs` - Input handling
- `src/math/` - Vector and matrix math utilities

## Dependencies

- `winit` - Window and event handling
- `pixels` - Framebuffer management

## Current State

Currently renders a wireframe cube with interactive camera and object rotation. The foundation is laid for expanding to more complex geometries and rendering techniques.
