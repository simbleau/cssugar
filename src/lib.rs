#![feature(type_alias_impl_trait)]

pub mod colors;
pub mod functions;
pub mod units;

pub mod prelude {
    pub use crate::colors::*;
    pub use crate::units::*;
}
