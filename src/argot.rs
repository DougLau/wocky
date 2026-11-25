use std::str::FromStr;

/// Argot (dialect, jargon or vernacular)
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Argot {
    /// Common, everyday
    Common,
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

impl FromStr for Argot {
    type Err = ();

    fn from_str(s: &str) -> Result<Argot, ()> {
        use Argot::*;
        match s {
            "common" => Ok(Common),
            "legal" => Ok(Legal),
            "scientific" => Ok(Scientific),
            "computing" => Ok(Computing),
            "botanical" => Ok(Botanical),
            "medical" => Ok(Medical),
            "offensive" => Ok(Offensive),
            "literary" => Ok(Literary),
            "archaic" => Ok(Archaic),
            _ => Err(()),
        }
    }
}

impl Argot {
    /// Get all argots
    pub fn all() -> &'static [Self] {
        use Argot::*;
        &[
            Common, Legal, Scientific, Computing, Botanical, Medical,
            Offensive, Literary, Archaic,
        ]
    }

    /// Get argot CSV as a string slice
    pub const fn csv(self) -> &'static str {
        use Argot::*;
        match self {
            Common => include_str!("../res/common.csv"),
            Legal => include_str!("../res/legal.csv"),
            Scientific => include_str!("../res/scientific.csv"),
            Computing => include_str!("../res/computing.csv"),
            Botanical => include_str!("../res/botanical.csv"),
            Medical => include_str!("../res/medical.csv"),
            Offensive => include_str!("../res/offensive.csv"),
            Literary => include_str!("../res/literary.csv"),
            Archaic => include_str!("../res/archaic.csv"),
        }
    }
}
