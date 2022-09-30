pub(crate) mod calculation;
pub use calculation::Calculation;

mod max;
pub use max::Max;

mod min;
pub use min::Min;

pub mod ops;

mod __markers;
pub(crate) mod markers {
    pub use super::__markers::Addable;
    pub use super::__markers::Maxable;
    pub use super::__markers::Minable;
    pub use super::__markers::Scalable;
}
