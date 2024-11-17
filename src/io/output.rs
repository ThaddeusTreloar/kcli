use std::{fmt::Display, str::FromStr};

use clap::{builder::PossibleValue, ValueEnum};
use error_stack::ResultExt;
use ron::ser::PrettyConfig;
use serde::Serialize;

use crate::error::io::OutputError;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Output {
    #[default]
    Human,
    Json,
    Yaml,
}

impl Output {
    pub fn output_string<T>(&self, value: &T) -> error_stack::Result<String, OutputError>
    where
        T: Serialize,
    {
        match self {
            Self::Human => ron::ser::to_string_pretty(value, PrettyConfig::default())
                .change_context(OutputError::Serialise),
            Self::Json => serde_json::to_string(value).change_context(OutputError::Serialise),
            Self::Yaml => serde_yml::to_string(value).change_context(OutputError::Serialise),
        }
    }
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
        &[Output::Human, Output::Json, Output::Yaml]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Output::Human => PossibleValue::new("human"),
            Output::Json => PossibleValue::new("json"),
            Output::Yaml => PossibleValue::new("yaml"),
        })
    }
}
