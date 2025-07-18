// Copyright 2025 Irreducible Inc.

use binius_field::TowerField;

use super::{
	ComputeLayerExecutor, ComputeMemory,
	alloc::ComputeAllocator,
	layer::{ComputeLayer, Error, FSliceMut},
};

/// Computes the partial evaluation of the equality indicator polynomial.
///
/// Given an $n$-coordinate point $r_0, ..., r_n$, this computes the partial evaluation of the
/// equality indicator polynomial $\widetilde{eq}(X_0, ..., X_{n-1}, r_0, ..., r_{n-1})$ and
/// returns its values over the $n$-dimensional hypercube.
///
/// The returned values are equal to the tensor product
///
/// $$
/// (1 - r_0, r_0) \otimes ... \otimes (1 - r_{n-1}, r_{n-1}).
/// $$
///
/// See [DP23], Section 2.1 for more information about the equality indicator polynomial.
///
/// [DP23]: <https://eprint.iacr.org/2023/1784>
pub fn eq_ind_partial_eval<'a, F, Hal, DeviceAllocatorType>(
	hal: &Hal,
	dev_alloc: &'a DeviceAllocatorType,
	point: &[F],
) -> Result<FSliceMut<'a, F, Hal>, Error>
where
	F: TowerField,
	Hal: ComputeLayer<F>,
	DeviceAllocatorType: ComputeAllocator<F, Hal::DevMem>,
{
	let n_vars = point.len();
	let mut out = dev_alloc.alloc(1 << n_vars)?;

	{
		let mut dev_val = Hal::DevMem::slice_power_of_two_mut(&mut out, 1);
		hal.fill(&mut dev_val, F::ONE)?;
	}

	hal.execute(|exec| {
		exec.tensor_expand(0, point, &mut out)?;
		Ok(vec![])
	})?;

	Ok(out)
}
