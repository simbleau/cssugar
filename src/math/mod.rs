pub(crate) mod calculation;
pub use calculation::Calculation;

pub(crate) mod max;
pub use max::Max;

pub(crate) mod min;
pub use min::Min;

pub mod ops;

mod __markers;
pub(crate) mod markers {
    pub use super::__markers::Addable;
    pub use super::__markers::Calculable;
    pub use super::__markers::Maxable;
    pub use super::__markers::Minable;
    pub use super::__markers::Scalable;
}
