use crate::prelude::*;

pub struct NoResample;

impl Resampler for NoResample {
    fn resample(&self, from: &[u32], to: &mut [u32]) {
        let up = from.len();
        let down = to.len();
        to.iter_mut().enumerate().map(|(idx, a)| (a, idx * up / down)).for_each(|(a, fidx)| *a = from[fidx]);
    }
}

pub mod prelude {
    pub use super::NoResample;
}

