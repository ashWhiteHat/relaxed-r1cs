use crate::commitment::CommitmentScheme;
use crate::committed_relaxed_r1cs::Instance as CommittedRelaxedR1csInstance;
use crate::matrix::DenseVectors;

use zkstd::common::{CurveAffine, PrimeField};

/// instance for relaxed r1cs (E, u, x)
#[derive(Clone, Debug)]
pub struct Instance<F: PrimeField> {
    /// error vectors
    pub(crate) e: DenseVectors<F>,
    /// scalar
    pub(crate) u: F,
    /// public inputs and outputs
    pub(crate) x: DenseVectors<F>,
}

impl<F: PrimeField> Instance<F> {
    pub(crate) fn new(m: usize, x: DenseVectors<F>) -> Self {
        Self {
            e: DenseVectors(vec![F::zero(); m]),
            u: F::one(),
            x,
        }
    }
}

pub(crate) fn commit_relaxed_r1cs_instance_data<C: CurveAffine>(
    relaxed_r1cs_instance: &Instance<C::Scalar>,
    w: &DenseVectors<C::Scalar>,
    cs: &CommitmentScheme<C>,
) -> CommittedRelaxedR1csInstance<C> {
    let Instance { e, u, x } = relaxed_r1cs_instance;
    CommittedRelaxedR1csInstance {
        overline_e: cs.commit(e, u),
        u: *u,
        overline_w: cs.commit(w, u),
        x: x.clone(),
    }
}