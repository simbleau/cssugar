#![feature(type_alias_impl_trait)]

pub mod colors;
pub mod dimensions;
pub mod functions;

pub mod prelude {
    pub use crate::colors::*;
    pub use crate::dimensions::*;
}
