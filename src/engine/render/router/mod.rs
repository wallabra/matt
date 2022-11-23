pub mod matrix;

use crate::prelude::*;
use rayon::prelude::*;

pub struct Router {
    mat: matrix::RoutingMatrix,
    sources: Vec<StereoBuffer>,
    sinks: Vec<StereoBuffer>,
}

impl Router {
    fn route(&mut self) {
        self.sinks
            .par_iter_mut()
            .enumerate()
            .for_each(|(sink_idx, sink)| {
                let added = self
                    .mat
                    .sources_for(sink_idx)
                    .map(|(src_idx, scale)| self.sources[src_idx].scaled(scale))
                    .reduce(StereoBuffer::add_buf_iters);

                if let Some(added) = added {
                    sink.set(added);
                }
            });
    }
}

pub mod prelude {
    pub use super::Router;

    pub use super::matrix::prelude::*;
}
