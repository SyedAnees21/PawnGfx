# PawnGFX

PawnGFX is a CPU-based software renderer built in Rust to explore 3D graphics fundamentals. The workspace now includes a headless renderer library, a scene crate, and an in-progress editor UI.

See the current pipeline diagrams and flow notes in `docs/ARCHITECTURE.md`.

## Overview

Core renderer capabilities:

- Software rasterization pipeline (triangles + depth)
- Camera + transforms (model/view/projection)
- Texturing + normal mapping
- Basic lighting/shading models
- Asset loading for meshes/textures

For the full feature list, see `docs/FEATURES.md`.

## Project Structure

```text
.
+- core/            # Math + geometry primitives used by other crates
+- scene/           # Scene data + assets + utilities (shared)
+- renderer/        # Software renderer library
+- editor/          # Editor UI (egui) #WIP
+- assets/          # Sample assets (meshes/textures)
+- docs/            # Architecture + design docs + mdBook
+- README.md
```

---

## Standalone Example

The renderer includes a standalone example that opens its own window and renders a default scene.

Run it with:

```bash
cargo run -p prenderer --example standalone --features standalone --release
```

### Controls (Standalone)

- `W/A/S/D/Q/E`: Move camera (forward, back, left, right, up, down)
- `Arrow Keys`: Rotate the object (up/down = X axis, left/right = Y axis)
- `Right click + Drag`: Camera rotation

---
