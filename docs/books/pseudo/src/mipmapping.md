# Mipmapping

This chapter explains how mipmaps are generated and how LOD is chosen for sampling.

## Mipmaps

### Theory
Mipmaps are prefiltered, downsampled versions of a texture. Each subsequent level is half the width and height of the previous level (down to 1x1). Sampling from lower-resolution levels reduces aliasing when a texture is minified in screen space.

### Pseudocode
```text
function build_mipmaps(base_image):
    mipmaps = [base_image]
    while width(mipmaps.last) > 1 or height(mipmaps.last) > 1:
        prev = mipmaps.last
        new_w = max(1, floor(prev.width / 2))
        new_h = max(1, floor(prev.height / 2))
        next = new_image(new_w, new_h)

        for y in 0..new_h-1:
            for x in 0..new_w-1:
                x0 = min(2*x,     prev.width  - 1)
                x1 = min(2*x + 1, prev.width  - 1)
                y0 = min(2*y,     prev.height - 1)
                y1 = min(2*y + 1, prev.height - 1)

                c0 = prev.texel(x0, y0)
                c1 = prev.texel(x1, y0)
                c2 = prev.texel(x0, y1)
                c3 = prev.texel(x1, y1)

                next(x, y) = (c0 + c1 + c2 + c3) / 4

        mipmaps.push(next)

    return mipmaps
```

## LOD Calculation

### Theory
LOD (level-of-detail) selects which mip level to use for a given pixel. A higher LOD means a lower-resolution mipmap. LOD is derived from **screen-space derivatives** of UVs. Because UVs are perspective-correct, we compute gradients on **pre-divided** values (`u' = u / w`, `v' = v / w`, `w' = 1 / w`) and then convert them into true derivatives using the chain rule.

This section is split into two steps:
1. **Derivative calculation** (screen-space gradients for `u'`, `v'`, `w'`).
2. **LOD calculation** (convert derivatives to texel space and compute `log2`).

### Derivatives Calculation

#### Theory
For a triangle, any attribute varies linearly in screen space. We can compute constant gradients `df/dx` and `df/dy` using the triangle’s screen coordinates. These gradients are computed for `u'`, `v'`, and `w'` once per triangle and reused for each pixel.

#### Pseudocode
```text
function gradient(f0, f1, f2, s0, s1, s2):
    area = edge_function(s0, s1, s2)
    if abs(area) < epsilon: return (0, 0)
    inv_area = 1 / area

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

### LOD From Derivatives

#### Theory
Given the gradients of `u'`, `v'`, and `w'`, we convert to true UV derivatives:

- `u = u' / w'`
- `v = v' / w'`

Using the quotient rule, we compute `du/dx`, `du/dy`, `dv/dx`, `dv/dy`, then convert to texel space and use the maximum gradient length for LOD.

#### Pseudocode
```text
function lod_from_derivatives(u', v', inv_w,
                              du_dx, du_dy,
                              dv_dx, dv_dy,
                              dw_dx, dw_dy,
                              tex_w, tex_h,
                              max_lod):
    if inv_w <= 0: return 0

    # Convert derivatives of u', v' (pre-divide) into true uv derivatives
    inv_w2 = inv_w * inv_w

    dux = (du_dx * inv_w - u' * dw_dx) / inv_w2
    duy = (du_dy * inv_w - u' * dw_dy) / inv_w2
    dvx = (dv_dx * inv_w - v' * dw_dx) / inv_w2
    dvy = (dv_dy * inv_w - v' * dw_dy) / inv_w2

    # Convert to texel space
    dudx = dux * tex_w
    dudy = duy * tex_w
    dvdx = dvx * tex_h
    dvdy = dvy * tex_h

    # Use the larger of the gradients for LOD
    rho = max( length(dudx, dvdx), length(dudy, dvdy), 1e-8 )
    lod = log2(rho)

    return clamp(lod, 0, max_lod)
```
