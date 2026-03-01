# Attribute Gradient Calculation and Incremental Perspective Interpolation

This document explains how attribute gradients are computed in screen space, how they can replace per‑pixel edge function evaluation, and how they enable incremental, perspective‑correct interpolation. It also provides a performance analysis for a CPU rasterizer like PawnGFX.

## 1. Background: What Is an Attribute Gradient?

For any attribute that varies linearly across a triangle in screen space (e.g., `u'`, `v'`, `1/w`, depth, color), the value at any pixel can be expressed as:

```text
A(x, y) = A0 + (dA/dx) * (x - x0) + (dA/dy) * (y - y0)
```

The partial derivatives `dA/dx` and `dA/dy` are **constant across the triangle**. If you compute them once, you can update the attribute incrementally as you march across pixels.

### Why this matters

- **No per‑pixel barycentric recomputation** for each attribute.
- **No repeated edge function evaluation** for interpolation.
- **Incremental updates** are cache‑friendly and branch‑light.

## 2. Computing Gradients (Screen‑Space)

Let screen‑space positions be `s0(x0, y0)`, `s1(x1, y1)`, `s2(x2, y2)` and attribute values `A0`, `A1`, `A2`.

The signed triangle area is:

```text
area = edge_function(s0, s1, s2)
```

Then:

```text
dA/dx = ( (A1 - A0) * (y2 - y0) - (A2 - A0) * (y1 - y0) ) / area

 dA/dy = (-(A1 - A0) * (x2 - x0) + (A2 - A0) * (x1 - x0) ) / area
```

These formulas are used for any scalar attribute: `u'`, `v'`, `1/w`, depth, or pre‑multiplied varyings.

## 3. Using Gradients Instead of Edge Functions

### Traditional approach (edge functions + barycentric weights)

At each pixel:

1. Compute three edge functions.
2. Normalize to get barycentric weights.
3. Interpolate each attribute with barycentric weights.

### Gradient approach

At triangle setup:

1. Compute `dA/dx` and `dA/dy` for each attribute once.
2. Evaluate attribute at the first pixel in a row.

At each pixel:

- Move right: `A += dA/dx`
- Move down a row: `A_row_start += dA/dy`

The edge function is still needed for **coverage** (inside/outside), but not for interpolation. If you also compute edge gradients, coverage can be tested incrementally as well.

### Optional: Incremental coverage

Edge functions themselves are linear, so they can be updated incrementally too:

```text
E(x+1, y) = E(x, y) + dE/dx
E(x, y+1) = E(x, y) + dE/dy
```

That lets you avoid recalculating edge functions per pixel entirely.

## 4. Perspective‑Correct Interpolation with Gradients

Perspective‑correct interpolation uses the identity:

```text
A = (A' / w')
where A' = A / w and w' = 1 / w
```

### Step‑by‑step

1. At vertices, compute:
   - `u' = u * inv_w`
   - `v' = v * inv_w`
   - `w' = inv_w`

2. Compute gradients for `u'`, `v'`, and `w'`.

3. Incrementally update `u'`, `v'`, and `w'` across pixels.

4. At each pixel, recover perspective‑correct UV:

```text
 u = u' / w'
 v = v' / w'
```

This avoids barycentric interpolation of `u`, `v` directly while remaining perspective‑correct.

### Incremental perspective interpolation loop

```text
# Setup for a scanline row
u_row = u' at row start
v_row = v' at row start
w_row = w' at row start

for each pixel in row:
    u = u_row / w_row
    v = v_row / w_row

    u_row += du'/dx
    v_row += dv'/dx
    w_row += dw'/dx

# Next row start
u_row_start += du'/dy
v_row_start += dv'/dy
w_row_start += dw'/dy
```

## 5. Performance Analysis

### Where the renderer gains

1. **Fewer FLOPs per pixel**
   - Barycentric interpolation needs 3 edge functions and 3 multiplies per attribute.
   - Gradients reduce this to 1–2 adds per attribute per pixel.

2. **Better cache and branch behavior**
   - Incremental stepping reduces branching and random access patterns.
   - Fewer math ops means more pixels per CPU cycle.

3. **Cheaper perspective correction**
   - You only divide once per pixel for each attribute (u/w, v/w).
   - You avoid recomputing barycentric weights for every attribute.

4. **Vectorization‑friendly**
   - Incremental updates are simple linear operations, which compilers can auto‑vectorize.
   - This is important for CPU rasterization workloads.

### Cost trade‑off

- Gradient setup is **per triangle**, not per pixel, so it amortizes well.
- For very tiny triangles (a few pixels), the benefit is smaller, but the overhead is still low.

### Expected speedup in a CPU rasterizer

- Typical gains are **10–30%** in the rasterization and interpolation stage, depending on triangle size distribution and attribute count.
- Additional gains come from incremental edge testing (coverage), if applied.

## 6. Practical Notes for PawnGFX

- You already compute gradients for LOD calculation. The same method can drive incremental interpolation for all varyings.
- If you adopt this approach, your raster loop becomes more scanline‑like and less barycentric‑heavy.
- A good path forward is:
  1. Add gradient setup for `u'`, `v'`, `w'`, depth, and any custom varyings.
  2. Replace per‑pixel barycentric interpolation with incremental updates.
  3. Optionally convert edge tests to incremental edge stepping.

---

If you want, I can also add a second doc with a concrete step‑by‑step rewrite of your current raster loop into an incremental form.
