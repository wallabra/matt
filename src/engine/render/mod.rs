pub mod router;
pub mod sampler;
pub mod voice;

pub trait SourceObj {
    fn gush_audio(&self, to: &mut [u32]);
}

pub trait SinkObj {
    fn drain_audio(&mut self, from: &[u32]);
}

pub mod prelude {
    pub use super::{SinkObj, SourceObj};

    pub use super::router::prelude::*;
    pub use super::sampler::prelude::*;
    pub use super::voice::prelude::*;
}
