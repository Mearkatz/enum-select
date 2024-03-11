use inquire::{InquireError, MultiSelect, Select};
use std::{fmt::Display, vec::Vec};
use strum::VariantArray;

trait Selectable: VariantArray + Display + Clone {
    fn select_one(message: &str) -> Select<'_, Self> {
        Select::new(message, Self::VARIANTS.to_vec())
    }
    fn select_multiple(message: &str) -> MultiSelect<'_, Self> {
        MultiSelect::new(message, Self::VARIANTS.to_vec())
    }
}
