/// Return the nearest power-of-two exponent for a given float exponent.
/// Values <= 1.0 map to 1.0. Ties pick the lower power of two.
pub fn nearest_pow2_u32(exp: f32) -> u32 {
	let e = exp;
	if !e.is_finite() || e <= 1.0 {
		return 1;
	}

	let e_int = e.round() as u32;
	if e_int <= 1 {
		return 1;
	}
	if e_int.is_power_of_two() {
		return e_int;
	}

	let upper = e_int.next_power_of_two();
	let lower = upper >> 1;
	if e_int - lower <= upper - e_int {
		lower
	} else {
		upper
	}
}

/// Fast power for exponents constrained to powers of two.
/// If `exp` isn't a power of two, it is snapped to the nearest power of two.
/// Uses repeated squaring instead of `powf`.
pub fn powf_pow2(base: f32, exp: f32) -> f32 {
	let pow2 = nearest_pow2_u32(exp);
	if pow2 == 0 {
		return 1.0;
	}

	let mut result = base;
	let mut n = pow2;
	while n > 1 {
		result *= result;
		n >>= 1;
	}
	result
}

/// Very fast approximate pow using float bit tricks.
/// This is only valid for base > 0 and is an approximation.
/// Good for rough specular when you want speed over accuracy.
pub fn fast_pow_approx(base: f32, exp: f32) -> f32 {
	if !base.is_finite() || base <= 0.0 {
		return 0.0;
	}

	// Approximate log2(base) via float bits, then scale by exp and rebuild.
	// This is a classic fast pow approximation; error can be noticeable.
	let x = base.to_bits() as i32;
	let y = (exp * (x - 1065353216) as f32) + 1065353216.0;
	f32::from_bits(y as u32)
}

/// Lookup-table based pow for x in [0, 1].
/// Precompute once per exponent to avoid powf in the hot loop.
pub struct PowLut {
	exp: f32,
	inv_step: f32,
	values: Vec<f32>,
}

impl PowLut {
	pub fn new(exp: f32, size: usize) -> Self {
		let size = size.max(2);
		let inv_step = (size - 1) as f32;
		let mut values = Vec::with_capacity(size);
		for i in 0..size {
			let x = i as f32 / inv_step;
			values.push(x.powf(exp));
		}
		Self {
			exp,
			inv_step,
			values,
		}
	}

	#[inline]
	pub fn exp(&self) -> f32 {
		self.exp
	}

	/// Sample the LUT with linear interpolation.
	#[inline]
	pub fn sample(&self, x: f32) -> f32 {
		let t = x.clamp(0.0, 1.0) * self.inv_step;
		let i0 = t.floor() as usize;
		let i1 = (i0 + 1).min(self.values.len() - 1);
		let frac = t - i0 as f32;
		let v0 = self.values[i0];
		let v1 = self.values[i1];
		v0 + (v1 - v0) * frac
	}
}
