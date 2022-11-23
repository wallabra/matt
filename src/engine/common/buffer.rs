pub struct StereoBuffer {
    pub left: Vec<u32>,
    pub right: Vec<u32>,
}

pub trait Resampler {
    fn resample(&self, from: &[u32], to: &mut [u32]);
}

impl StereoBuffer {
    pub fn new(length: usize) -> StereoBuffer {
        StereoBuffer {
            left: vec![0; length],
            right: vec![0; length],
        }
    }

    pub fn add(&mut self, values: (Vec<u32>, Vec<u32>)) {
        for (lo, li) in self.left.iter_mut().zip(values.0.into_iter()) {
            *lo += li;
        }

        for (ro, ri) in self.right.iter_mut().zip(values.1.into_iter()) {
            *ro += ri;
        }
    }

    pub fn set(&mut self, values: (Vec<u32>, Vec<u32>)) {
        for (lo, li) in self.left.iter_mut().zip(values.0.into_iter()) {
            *lo = li;
        }

        for (ro, ri) in self.right.iter_mut().zip(values.1.into_iter()) {
            *ro = ri;
        }
    }

    pub fn scaled<'a>(
        &'a self,
        scale: f32,
    ) -> (
        Box<dyn Iterator<Item = u32> + 'a>,
        Box<dyn Iterator<Item = u32> + 'a>,
    ) {
        let scaler = move |x: &u32| (*x as f64 * scale as f64) as u32;

        (
            Box::new(self.left.iter().map(scaler)),
            Box::new(self.right.iter().map(scaler)),
        )
    }

    pub fn add_buf_iters<'a, 'b, 'c>(
        a: (
            Box<dyn Iterator<Item = u32> + 'a>,
            Box<dyn Iterator<Item = u32> + 'a>,
        ),
        b: (
            Box<dyn Iterator<Item = u32> + 'b>,
            Box<dyn Iterator<Item = u32> + 'b>,
        ),
    ) -> (
        Box<dyn Iterator<Item = u32> + 'c>,
        Box<dyn Iterator<Item = u32> + 'c>,
    )
    where
        'a: 'c,
        'b: 'c,
    {
        (
            Box::new(a.0.zip(b.0).map(|(a, b)| a + b)),
            Box::new(a.1.zip(b.1).map(|(a, b)| a + b)),
        )
    }

    pub fn copy_to_and_resample(
        &self,
        to: (&mut [u32], &mut [u32]),
        resampler: Box<impl Resampler>,
    ) {
        debug_assert!(to.0.len() == to.1.len());
        debug_assert!(self.left.len() == self.right.len());

        if self.left.len() == to.0.len() {
            to.0.iter_mut()
                .zip(self.left.iter())
                .for_each(|(os, is)| *os = *is);
            to.1.iter_mut()
                .zip(self.right.iter())
                .for_each(|(os, is)| *os = *is);
        } else {
            resampler.resample(&self.left, to.0);
            resampler.resample(&self.right, to.1);
        }
    }
}

impl<'a> From<&'a StereoBuffer> for (&'a [u32], &'a [u32]) {
    fn from(val: &'a StereoBuffer) -> (&'a [u32], &'a [u32]) {
        (&val.left, &val.right)
    }
}

impl<'a> From<&'a mut StereoBuffer> for (&'a mut [u32], &'a mut [u32]) {
    fn from(val: &'a mut StereoBuffer) -> (&'a mut [u32], &'a mut [u32]) {
        (&mut val.left, &mut val.right)
    }
}

pub mod prelude {
    pub use super::{Resampler, StereoBuffer};
}
