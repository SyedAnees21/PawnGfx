# PawnGFX Feature Checklist

## Core Rendering

- [x] Framebuffer rendering
- [x] Line rasterization (Bresenham's algorithm)
- [x] Wireframe cube rendering
- [x] Triangle rasterization
  - [x] Bounding box (naive)
  - [ ] Scan line (advance)
- [x] Face filling / solid rendering
- [x] Texture mapping
  - [x] Linear sampling
  - [x] Bi-Linear sampling
  - [x] Wrapping
- [x] Shaders
  - [x] Vertex
  - [x] Fragment
- [ ] Shading models (flat, Gouraud, Phong)
  - [x] Flat-Lambart
  - [ ] Gouraud
  - [ ] Blinn-Phong
- [x] Normal Maps
  - [x] TBN matrix
  - [x] Sampling (linear/bilinear)
- [ ] Shadow rendering
- [ ] Anti-aliasing

## 3D Mathematics

- [x] Vector3 operations (add, subtract, multiply, dot, cross, normalize)
- [x] Vector4 operations
- [x] Matrix4 operations (identity, transpose, multiplication)
- [x] Matrix3 operations (identity, transpose, multiplication)
- [x] Transformation matrices (translation, rotation, scaling)
- [x] Perspective projection matrix
- [ ] View frustum culling
- [x] Matrix inverse
- [ ] Quaternion support

## Camera System

- [x] Perspective camera
- [x] Camera movement (forward, backward, left, right)
- [x] Camera rotation (yaw, pitch)
- [ ] Camera roll
- [ ] Orthographic projection
- [ ] Fly-through camera
- [ ] Orbital camera

## Input Handling

- [x] Basic Input state machine
- [x] Keyboard input (WASD + arrows)
- [x] Mouse input detection
- [x] Mouse movement tracking
- [X] Mouse camera control
- [ ] Relative mouse movement
- [ ] Gamepad support

## Geometry & Objects

- [x] Cube rendering
- [ ] Sphere rendering
- [ ] Plane rendering
- [ ] Pyramid rendering
- [x] Custom mesh loading
  - [x] Normal baking
  - [x] Tangents baking
- [ ] Model file support (OBJ, glTF, etc.)
  - [x] OBJ
  - [ ] glTF
- [ ] Mesh instancing

## Performance & Optimization

- [x] Depth buffer
- [x] Z-buffer optimization
- [x] Back-face culling
- [ ] Frustum culling
- [ ] Level-of-detail (LOD)
- [ ] Multi-threading support

### Profiling Checklist (CPU Renderer)

- [ ] Build in `--release` with LTO enabled
- [ ] Profile with `cargo flamegraph` or platform profiler (Windows Performance Analyzer)
- [ ] Measure time split: raster loop vs fragment shading vs texture sampling
- [ ] Check `powf`/`sqrt` hot spots in fragment shaders
- [ ] Validate math precision (`f32` vs `f32`) in hot loops
- [ ] Minimize allocations in per-frame code paths
- [ ] Confirm early‑exit logic (backface cull, depth test) triggers as expected
- [ ] Measure texture sampling cost (bi‑linear, normal maps)
- [ ] Evaluate gradient interpolation vs barycentric cost
- [ ] Check cache behavior: stride in framebuffer and depth buffer

## Lighting & Effects

- [x] Ambient lighting
- [x] Directional lights
- [ ] Point lights
- [ ] Spot lights
- [ ] Normal mapping
- [ ] Parallax mapping
- [ ] Bloom effects
- [ ] Fog

## UI & Visualization

- [ ] Framerate counter
- [ ] Debug visualization
- [ ] Wireframe/solid mode toggle
- [ ] Lighting visualization
- [ ] Grid overlay

## Window Management

- [x] Resizable window
- [x] Window event handling
- [ ] Full-screen support
- [ ] V-sync control
- [ ] Frame rate limiting

## Configuration

- [ ] Settings file support
- [ ] Runtime configuration
- [ ] Customizable resolution
- [ ] Customizable camera speed
- [ ] Input remapping
