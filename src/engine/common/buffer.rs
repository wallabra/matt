pub struct StereoBuffer {
    length: usize,
    left: Vec<u32>,
    right: Vec<u32>,
}

impl StereoBuffer {
    pub fn new(length: usize) -> StereoBuffer {
        StereoBuffer {
            length,
            left: vec![0; length],
            right: vec![0; length],
        }
    }

    pub fn add(&mut self, values: (Box<dyn Iterator<Item = u32>>, Box<dyn Iterator<Item = u32>>)) {
        for (lo, li) in self.left.iter_mut().zip(values.0) {
            *lo += li;
        }

        for (ro, ri) in self.right.iter_mut().zip(values.1) {
            *ro += ri;
        }
    }

    pub fn set(&mut self, values: (Box<dyn Iterator<Item = u32>>, Box<dyn Iterator<Item = u32>>)) {
        for (lo, li) in self.left.iter_mut().zip(values.0) {
            *lo = li;
        }

        for (ro, ri) in self.right.iter_mut().zip(values.1) {
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
}

pub mod prelude {
    pub use super::StereoBuffer;
}
