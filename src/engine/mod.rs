pub mod common;
pub mod midi;
pub mod render;

pub struct Engine {}

pub mod prelude {
    pub use super::Engine;

    pub use super::common::prelude::*;
    pub use super::midi::prelude::*;
    pub use super::render::prelude::*;
}
