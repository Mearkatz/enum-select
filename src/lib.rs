use inquire::{MultiSelect, Select};
use std::fmt::Display;
use strum::VariantArray;

pub trait Selectable: VariantArray + Display + Clone {
    fn select_one(message: &str) -> Select<'_, Self> {
        Select::new(message, Self::VARIANTS.to_vec())
    }
    fn select_multiple(message: &str) -> MultiSelect<'_, Self> {
        MultiSelect::new(message, Self::VARIANTS.to_vec())
    }
}
