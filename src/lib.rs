use inquire::{InquireError, MultiSelect, Select};
use std::{fmt::Display, str::FromStr, vec::Vec};
use strum::VariantArray;

pub trait SelectVariant: VariantArray + Display + FromStr + Clone {
    /// Selects a single variant from this Enum.
    /// # Errors
    ///
    /// Will return `Err` if no variant is selected or there is a problem getting user input from the terminal.

    fn select_one_variant(message: &str) -> Result<Self, InquireError> {
        Select::new(message, Self::VARIANTS.to_vec()).prompt()
    }

    /// Selects multiple variants from this Enum.
    /// # Errors
    ///
    /// Will return `Err` if no variants are selected or there is a problem getting user input from the terminal.
    fn select_multiple_variants(message: &str) -> Result<Box<[Self]>, InquireError> {
        MultiSelect::new(message, Self::VARIANTS.to_vec())
            .prompt()
            .map(Vec::into_boxed_slice)
    }
}
