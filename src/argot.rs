/// Argot (dialect, jargon or vernacular)
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Argot {
    /// Normal, everyday
    Normal,
    /// Legal jargon
    Legal,
    /// Scientific jargon
    Scientific,
    /// Computing jargon
    Computing,
    /// Botanical jargon
    Botanical,
    /// Medical jargon
    Medical,
    /// Offensive words
    Offensive,
    /// Literary (rarely used in everyday speech)
    Literary,
    /// Archaic (rarely used any more)
    Archaic,
}

impl Argot {
    pub fn all() -> &'static [Self] {
        use Argot::*;
        &[
            Normal, Legal, Scientific, Computing, Botanical, Medical,
            Offensive, Literary, Archaic,
        ]
    }
}
