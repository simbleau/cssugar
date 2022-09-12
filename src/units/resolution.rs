#[derive(Debug, Clone, PartialEq)]
pub enum Resolution {
    Dpi(f64),
    Dpcm(f64),
    Dppx(f64),
}

impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Resolution::Dpi(v) => write!(f, "{}dpi", v),
            Resolution::Dpcm(v) => write!(f, "{}dpcm", v),
            Resolution::Dppx(v) => write!(f, "{}dppx", v),
        }
    }
}
