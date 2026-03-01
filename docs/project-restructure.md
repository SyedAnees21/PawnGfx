# Project Restructure Proposal

This document proposes a pragmatic restructuring of the PawnGFX project to improve clarity, separation of concerns, and long‑term scalability. It is written to be incremental: you can adopt it module by module without a full rewrite.

## Goals

- Make ownership and responsibilities explicit (input vs. scene vs. render vs. assets).
- Reduce cross‑module coupling.
- Group code by domain (math, geometry, rendering, IO).
- Prepare for future growth (materials, multiple objects, more shaders).

## Current Shape (High Level)

```
src/
  animate.rs
  color.rs
  draw.rs
  engine.rs
  error.rs
  input.rs
  main.rs
  raster.rs
  render.rs
  scene/
  geometry/
  loaders/
  math/
  shaders/
```

## Proposed Layout (Incremental)

```
src/
  app/
    mod.rs
    engine.rs
    loop.rs
    input.rs
    time.rs

  core/
    error.rs
    config.rs
    color.rs
    types.rs

  math/
    mod.rs
    vector.rs
    matrices.rs
    interpolate.rs

  geometry/
    mod.rs
    mesh.rs
    vertex.rs
    triangle.rs
    tangent.rs
    shapes.rs

  assets/
    mod.rs
    loaders/
      mod.rs
      obj.rs
      gltf.rs
    texture/
      mod.rs
      normal_map.rs
      mipmap.rs

  render/
    mod.rs
    renderer.rs
    raster/
      mod.rs
      triangle.rs
      coverage.rs
      interpolation.rs
    shaders/
      mod.rs
      effects.rs
      inputs.rs
    materials/
      mod.rs
      basic.rs

  scene/
    mod.rs
    camera.rs
    light.rs
    object.rs
    transform.rs
    animator.rs

  ui/
    mod.rs
    debug.rs

  main.rs
```

### Key Ideas

- **`app/`**: loop orchestration and input handling.
- **`core/`**: shared types, config, error, color.
- **`assets/`**: all file IO + texture/normal map logic.
- **`render/`**: rendering pipeline, rasterizer, shaders, materials.
- **`scene/`**: world‑state only, no IO or render logic.

## Suggested File Moves (Mapping)

**App / Loop**

- `src/engine.rs` -> `src/app/engine.rs`
- `src/input.rs` -> `src/app/input.rs`
- `src/animate.rs` -> `src/scene/animator.rs`

**Core / Shared**

- `src/color.rs` -> `src/core/color.rs`
- `src/error.rs` -> `src/core/error.rs`

**Geometry**

- `src/geometry/*` stays, optionally split tangents into `geometry/tangent.rs`.

**Assets / IO**

- `src/loaders/*` -> `src/assets/loaders/*`
- `src/scene/texture.rs` -> `src/assets/texture/mod.rs`
  - split mipmap logic into `src/assets/texture/mipmap.rs`
  - normal map conversion into `src/assets/texture/normal_map.rs`

**Rendering**

- `src/render.rs` -> `src/render/renderer.rs`
- `src/raster.rs` -> `src/render/raster/triangle.rs`
- `src/shaders/*` -> `src/render/shaders/*`
- `src/draw.rs` -> `src/render/raster/coverage.rs` (or `render/debug.rs` if only for wireframe)

**Scene**

- `src/scene/*` stays, but keep it data‑only.

## Practical Benefits

- **Fewer dependency cycles**: renderer won’t depend on loaders or scene IO.
- **Clear ownership**: assets loading is separate from render or scene.
- **Better testability**: you can test raster and shader modules in isolation.
- **Easier feature growth**: materials and multiple objects fit naturally.

## Suggested Module Responsibilities

**`render/`**

- `renderer.rs`: frame orchestration, buffer setup, entry point into raster.
- `raster/triangle.rs`: triangle traversal, interpolation, depth, coverage.
- `raster/interpolation.rs`: gradients and perspective interpolation helpers.
- `shaders/inputs.rs`: structures for varyings and attributes.
- `shaders/effects.rs`: concrete shader implementations.
- `materials/`: bindings between textures + shader params.

**`assets/texture/`**

- `mod.rs`: `Texture<T>`, sampling interface, wrapping.
- `mipmap.rs`: mip generation, LOD sampling strategies.
- `normal_map.rs`: normal decoding and conversions.

**`scene/`**

- `object.rs`: mesh + material + transform.
- `camera.rs`, `light.rs`, `transform.rs`: data components.
- `animator.rs`: scene update logic.

## Migration Steps (Low Risk)

1. Create new modules, re‑export to keep old paths working.
2. Move `render` modules first (`render.rs`, `raster.rs`, `shaders/`).
3. Move assets and loaders next (`scene/texture.rs`, `loaders/`).
4. Move engine/input into `app/`.
5. Clean imports and remove re‑exports.

## Optional Next Step: Data‑Driven Scene

If you want to grow beyond a single object, consider:

- a `SceneGraph` or simple ECS
- `Vec<Object>` instead of single `Object`

---

If you want, I can map your current paths into this structure automatically and generate a concrete move plan (with `git mv` commands) or keep it as a manual guide.
