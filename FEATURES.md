# PawnGFX Feature Checklist

## Core Rendering
- [x] Framebuffer rendering
- [x] Line rasterization (Bresenham's algorithm)
- [x] Wireframe cube rendering
- [x] Triangle rasterization
- [x] Face filling / solid rendering
- [ ] Texture mapping
- [ ] Shading models (flat, Gouraud, Phong)
- [ ] Shadow rendering
- [ ] Anti-aliasing

## 3D Mathematics
- [x] Vector3 operations (add, subtract, multiply, dot, cross, normalize)
- [x] Vector4 operations
- [x] Matrix4 operations (identity, transpose, multiplication)
- [x] Transformation matrices (translation, rotation, scaling)
- [x] Perspective projection matrix
- [ ] View frustum culling
- [ ] Matrix inverse
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
- [ ] Custom mesh loading
- [ ] Model file support (OBJ, glTF, etc.)
- [ ] Mesh instancing

## Performance & Optimization
- [x] Depth buffer
- [x] Z-buffer optimization
- [x] Back-face culling
- [ ] Frustum culling
- [ ] Level-of-detail (LOD)
- [ ] Multi-threading support

## Lighting & Effects
- [ ] Ambient lighting
- [ ] Directional lights
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
