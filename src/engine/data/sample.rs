use crate::prelude::*;

pub enum LoopDirection {
    Normal,
    PingPong,
}

pub struct LoopInfo {
    pub dir: LoopDirection,
    pub sustain: bool,
}

pub enum SampleDataBuffer {
    Mono(Vec<u32>),
    Stereo(Vec<u32>, Vec<u32>),
}

impl From<&SampleDataBuffer> for StereoBuffer {
    fn from(buf: &SampleDataBuffer) -> StereoBuffer {
        match buf {
            SampleDataBuffer::Mono(buf) => {
                let mut res = StereoBuffer::new(buf.len());
                res.left.copy_from_slice(buf);
                res.right.copy_from_slice(buf);
                res
            }

            SampleDataBuffer::Stereo(left, right) => {
                let mut res = StereoBuffer::new(left.len());
                res.left.copy_from_slice(left);
                res.right.copy_from_slice(right);
                res
            }
        }
    }
}

pub struct SampleData {
    pub looper: Option<LoopInfo>,
    pub loop_points: Option<(usize, usize)>,
    pub data: SampleDataBuffer,
    pub length: usize,
    pub samplerate: usize,
}

pub mod prelude {
    pub use super::{LoopDirection, LoopInfo, SampleData, SampleDataBuffer};
}
