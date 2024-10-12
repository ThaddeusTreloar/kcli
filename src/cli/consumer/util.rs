use std::{fmt::Display, str::FromStr};

use clap::{builder::PossibleValue, ValueEnum};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ResetStrategy {
    Beginning,
    Earliest,
    End,
    Largest,
    #[default]
    Latest,
    Smallest,
}

impl Display for ResetStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl FromStr for ResetStrategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}

impl ValueEnum for ResetStrategy {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Beginning,
            Self::Earliest,
            Self::End,
            Self::Largest,
            Self::Latest,
            Self::Smallest,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Beginning => PossibleValue::new("beginning"),
            Self::Earliest => PossibleValue::new("earliest"),
            Self::End => PossibleValue::new("end"),
            Self::Largest => PossibleValue::new("largest"),
            Self::Latest => PossibleValue::new("latest"),
            Self::Smallest => PossibleValue::new("smallest"),
        })
    }
}