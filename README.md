# PawnGFX

A minimal software renderer written in Rust for learning and experimentation with 3D graphics fundamentals.
The aim is to develop a fast and optimized light weight CPU based software renderer to view .obj and gltf models.

## Overview

PawnGFX is a barebone graphics renderer that implements core 3D graphics concepts including:
- Software-based rasterization
- Camera system with perspective projection
- 3D transformations (rotation, translation, scaling)
- Wireframe rendering of 3D objects
- User input handling for object and camera control

For update features list refer to [features](./docs/FEATURES.md)

## Features

- **3D Math Library**: Vector3, Vector4, and Matrix4 implementations with essential operations
- **Camera System**: Perspective camera with smooth movement and rotation
- **Wireframe Rendering**: Line-based drawing using Bresenham's line algorithm
- **Interactive Controls**: Keyboard input for camera movement and object rotation
- **Depth Management**: Depth buffer for proper rendering

## Building & Running

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run --release
```

## Controls

- **W/A/S/D/Q/E**: Move camera (forward, back, left, right, up, down)
- **Arrow Keys**: Rotate cube (↑/↓ for X-axis, ←/→ for Y-axis)
- **Right client + Mouse drag**: Camera rotation

