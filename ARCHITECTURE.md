# Architecture: PawnGFX Rendering Flow

## High-Level Loop
1. Input Phase
- `Engine::start_internal_loop` updates `InputState` from `winit` events.
- Mouse deltas and key states are collected for the frame.

2. Update Phase
- `ProceduralAnimator` updates camera position if active.
- Otherwise input is applied to camera and object transform.

3. Render Phase
- `Renderer::render` builds `model`, `view`, `projection`, and `normal` matrices.
- `GlobalUniforms` are assembled (matrices, light, camera, screen size).
- `raster::draw_call_generic` runs the software pipeline.

4. Present Phase
- Framebuffer is presented via `pixels::Pixels`.

## Software Shader Pipeline
1. Vertex Shader
- Input: `VertexIn` (position, normal, uv, face_normal).
- Output: `VertexOut` (clip position + varyings).

2. Rasterization
- Clip-space to NDC to screen-space.
- Back-face culling in screen-space.
- Barycentric rasterization within triangle bounds.
- Perspective-correct interpolation of varyings.

3. Fragment Shader
- Input: interpolated `Varyings` (uv, normal, world_pos, intensity).
- Output: final `Color`.

## Shader Effects
- `Flat`: per-face normal lighting.
- `Gouraud`: per-vertex diffuse lighting, interpolated intensity.
- `Phong`: per-fragment lighting with diffuse + specular.

## Diagram
```text
Input Events (winit)
    |
    v
InputState  ---->  Update Phase (Animator / Input)
    |                         |
    v                         v
Scene (Camera, Object, Light, Texture, Mesh)
    |
    v
Renderer::render
    |
    v
GlobalUniforms (matrices, light, camera, screen)
    |
    v
raster::draw_call_generic
    |
    v
Triangles iterator (positions + normals + uvs)
    |
    v
Vertex Shader (Flat | Gouraud | Phong)
    |
    v
Rasterization
  - clip -> NDC -> screen
  - back-face cull
  - barycentric + depth
  - perspective-correct varyings
    |
    v
Fragment Shader (Flat | Gouraud | Phong)
    |
    v
Framebuffer + DepthBuffer
    |
    v
pixels::Pixels::render (present)
```
