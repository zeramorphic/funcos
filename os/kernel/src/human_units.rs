use core::fmt::Display;

/// Displays this value approximately using binary prefixes, such as `4 Ki`.
/// The currently used prefixes are:
/// * `Ki = 2^10`
/// * `Mi = 2^20`
/// * `Gi = 2^30`
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BinaryPrefixes<T>(pub T);

impl BinaryPrefixes<usize> {
    pub fn display(&self, suffix: &str, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.0 < 10 << 10 {
            write!(f, "{} {suffix}", self.0)
        } else if self.0 < 10 << 20 {
            write!(f, "{} Ki{suffix}", self.0 >> 10)
        } else if self.0 < 10 << 30 {
            write!(f, "{} Mi{suffix}", self.0 >> 20)
        } else {
            write!(f, "{} Gi{suffix}", self.0 >> 30)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HumanBytes(pub usize);

impl Display for HumanBytes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        BinaryPrefixes(self.0).display("B", f)
    }
}
