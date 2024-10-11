use std::{fmt::Display, str::FromStr};

use clap::{builder::PossibleValue, ValueEnum};



#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub (crate) enum Output {
    Human,
    Json,
    Yaml,
}


impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl FromStr for Output {
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

impl ValueEnum for Output {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Output::Human,
            Output::Json,
            Output::Yaml,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Output::Human => PossibleValue::new("human"),
            Output::Json => PossibleValue::new("json"),
            Output::Yaml => PossibleValue::new("yaml"),
        })
    }
}