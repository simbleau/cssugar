pub mod colors;
pub mod dimensions;
pub mod math;

pub mod prelude {
    pub use crate::colors::*;
    pub use crate::dimensions::Angle::*;
    pub use crate::dimensions::Length::*;
    pub use crate::dimensions::Resolution::*;
    pub use crate::dimensions::Time::*;
    pub use crate::dimensions::*;
    pub use crate::math::ops::*;
}
