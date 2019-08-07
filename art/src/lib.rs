//! # Art
//!
//! A library for modelling artistic concepts.

pub use self::kinds::PrimaryColour;
pub use self::kinds::SecondaryColour;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colours according to RYB
    pub enum PrimaryColour {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colours according to RYB
    #[derive(PartialEq, Debug)]
    pub enum SecondaryColour {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colours to create a secondary colour.
    /// Combining the same primary colour with itself will give `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let red = art::PrimaryColour::Red;
    /// let yellow = art::PrimaryColour::Yellow;
    /// let orange = art::SecondaryColour::Orange;
    ///
    /// assert_eq!(Some(orange), art::mix(red, yellow));
    /// ```
    ///
    /// ```
    /// let red = art::PrimaryColour::Red;
    /// let other_red = art::PrimaryColour::Red;
    ///
    /// assert_eq!(None, art::mix(red, other_red));
    /// ```
    pub fn mix(col1: PrimaryColour, col2: PrimaryColour) -> Option<SecondaryColour> {
        match (col1, col2) {
            (PrimaryColour::Red, PrimaryColour::Yellow) => Some(SecondaryColour::Orange),
            (PrimaryColour::Red, PrimaryColour::Blue) => Some(SecondaryColour::Purple),
            (PrimaryColour::Yellow, PrimaryColour::Red) => Some(SecondaryColour::Orange),
            (PrimaryColour::Yellow, PrimaryColour::Blue) => Some(SecondaryColour::Green),
            (PrimaryColour::Blue, PrimaryColour::Red) => Some(SecondaryColour::Purple),
            (PrimaryColour::Blue, PrimaryColour::Yellow) => Some(SecondaryColour::Green),
            _ => None,
        }
    }
}
