# Derivative-Based LOD for Mipmapping

This algorithm computes a per-pixel LOD using screen-space derivatives of perspective-correct UVs. It then uses trilinear sampling to blend between mip levels.

## Inputs
- Triangle screen positions: `s0, s1, s2`
- UVs (already multiplied by `inv_w` per vertex): `uv0', uv1', uv2'`
- `inv_w` per vertex: `w0', w1', w2'` (where `w' = 1 / clip.w`)
- Base texture size: `tex_w`, `tex_h`
- Max mip level: `max_lod`

## Pseudocode
```text
# Precompute constant gradients for the triangle (screen-space)
area = edge_function(s0, s1, s2)
if abs(area) < epsilon: return
inv_area = 1 / area

# Attribute gradients in screen space (u', v', w')
(du_dx, du_dy) = gradient(uv0'.x, uv1'.x, uv2'.x, s0, s1, s2, inv_area)
(dv_dx, dv_dy) = gradient(uv0'.y, uv1'.y, uv2'.y, s0, s1, s2, inv_area)
(dw_dx, dw_dy) = gradient(w0',   w1',   w2',   s0, s1, s2, inv_area)

# Per pixel
for each pixel (x, y) inside triangle:
    bary = barycentric(s0, s1, s2, x, y)

    # perspective-correct inv_w and z
    inv_w = lerp(bary, w0', w1', w2')
    if inv_w <= 0: continue

    # perspective-correct varyings
    u' = lerp(bary, uv0'.x, uv1'.x, uv2'.x)
    v' = lerp(bary, uv0'.y, uv1'.y, uv2'.y)

    # Derivatives of true UV (u = u'/inv_w, v = v'/inv_w)
    inv_w2 = inv_w * inv_w

    du_dx = (du_dx * inv_w - u' * dw_dx) / inv_w2
    du_dy = (du_dy * inv_w - u' * dw_dy) / inv_w2
    dv_dx = (dv_dx * inv_w - v' * dw_dx) / inv_w2
    dv_dy = (dv_dy * inv_w - v' * dw_dy) / inv_w2

    # Convert to texel space
    dudx = du_dx * tex_w
    dudy = du_dy * tex_w
    dvdx = dv_dx * tex_h
    dvdy = dv_dy * tex_h

    # Use max gradient length for LOD
    rho = max( length(dudx, dvdx), length(dudy, dvdy), 1e-8 )
    lod = clamp( log2(rho), 0, max_lod )

    # Trilinear sampling
    color = sample_trilinear(u, v, lod)
```

## Helper: Screen-Space Gradient
```text
function gradient(f0, f1, f2, s0, s1, s2, inv_area):
    # f is any attribute at triangle vertices
    dx20 = s2.x - s0.x
    dy20 = s2.y - s0.y
    dx10 = s1.x - s0.x
    dy10 = s1.y - s0.y

    df10 = f1 - f0
    df20 = f2 - f0

    df_dx = (df10 * dy20 - df20 * dy10) * inv_area
    df_dy = (-df10 * dx20 + df20 * dx10) * inv_area

    return (df_dx, df_dy)
```

## Notes
- This is a **single-sample** anisotropic approximation. For higher quality anisotropic filtering, take multiple samples along the major axis.
- If the triangle is tiny (area near zero), skip to avoid division instability.
- If you clamp LOD, trilinear sampling remains stable.
