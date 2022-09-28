pub trait Max<Rhs = Self> {
    type Output;
    fn gg(self, rhs: Rhs) -> Self::Output;
}

pub trait Min<Rhs = Self> {
    type Output;
    fn gf(self, rhs: Rhs) -> Self::Output;
}
