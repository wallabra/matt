pub mod matrix;

use crate::prelude::*;

pub struct Router {
    bufsize: usize,
    mat: matrix::RoutingMatrix,
    sources: Vec<StereoBuffer>,
    sinks: Vec<StereoBuffer>,
}

impl Router {
    pub fn new(bufsize: usize) -> Self {
        Router {
            bufsize,
            mat: matrix::RoutingMatrix::default(),
            sources: vec![],
            sinks: vec![],
        }
    }

    pub fn add_source(&mut self) -> usize {
        self.sources.push(StereoBuffer::new(self.bufsize));
        self.sources.len() - 1
    }

    pub fn add_sink(&mut self) -> usize {
        self.sinks.push(StereoBuffer::new(self.bufsize));
        self.sinks.len() - 1
    }

    pub fn set_link(&mut self, source: usize, sink: usize, value: f32) {
        self.mat.set_link(source, sink, value);
    }

    fn get_added(&self, sink_idx: usize) -> Option<(Vec<u32>, Vec<u32>)> {
        self.mat
            .sources_for(sink_idx)
            .map(|(src_idx, scale)| self.sources[src_idx].scaled(scale))
            .reduce(StereoBuffer::add_buf_iters)
            .map(|x| (x.0.collect::<Vec<_>>(), x.1.collect::<Vec<_>>()))
    }

    pub fn route(&mut self) {
        let all_added = (0..self.sinks.len())
            .map(|x| self.get_added(x))
            .collect::<Vec<_>>();

        self.sinks
            .iter_mut()
            .zip(all_added.into_iter())
            .for_each(|(sink, added)| {
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
