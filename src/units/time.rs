#[derive(Debug, Clone, PartialEq)]
pub enum Time {
    Seconds(f64),
    Milliseconds(f64),
    Percent(f64),
    Duration(std::time::Duration),
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Time::Seconds(v) => write!(f, "{}s", v),
            Time::Milliseconds(v) => write!(f, "{}ms", v),
            Time::Percent(v) => write!(f, "{}%", v),
            Time::Duration(v) => write!(f, "{}ms", v.as_millis()),
        }
    }
}
