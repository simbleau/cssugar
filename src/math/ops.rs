pub trait Max<Rhs = Self> {
    type Output;
    fn max(self, rhs: Rhs) -> Self::Output;
}

pub trait Min<Rhs = Self> {
    type Output;
    fn min(self, rhs: Rhs) -> Self::Output;
}
