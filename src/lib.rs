use inquire::{InquireError, MultiSelect, Select};
use std::str::FromStr;
use strum::VariantNames;

trait SelectVariant: VariantNames + FromStr {
    /// Selects a single variant from this Enum.
    fn select_one_variant(message: &str) -> Result<Self, InquireError> {
        Select::new(message, Self::VARIANTS.to_vec())
            .prompt()
            .map(|x| unsafe { Self::from_str(x).unwrap_unchecked() })
    }

    /// Selects multiple variants from this Enum.
    fn select_multiple_variants(message: &str) -> Result<Box<[Self]>, InquireError> {
        MultiSelect::new(message, Self::VARIANTS.to_vec())
            .prompt()
            .map(|x| {
                x.into_iter()
                    .map(|s| unsafe { Self::from_str(s).unwrap_unchecked() })
                    .collect()
            })
    }
}
