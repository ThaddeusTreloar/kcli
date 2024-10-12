use std::{fmt::Display, str::FromStr};

use clap::{builder::PossibleValue, ValueEnum};
use error_stack::ResultExt;
use ron::ser::PrettyConfig;
use serde::Serialize;

use crate::error::io::InputError;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Input {
    #[default]
    Json,
    Yaml,
}

impl Input {
    pub fn write_as_string<T>(&self, value: &T) -> error_stack::Result<String, InputError>
    where
        T: Serialize,
    {
        match self {
            Self::Json => serde_json::to_string(value).change_context(InputError::Deserialise),
            Self::Yaml => serde_yml::to_string(value).change_context(InputError::Deserialise),
        }
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl FromStr for Input {
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

impl ValueEnum for Input {
    fn value_variants<'a>() -> &'a [Self] {
        &[Input::Json, Input::Yaml]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Input::Json => PossibleValue::new("json"),
            Input::Yaml => PossibleValue::new("yaml"),
        })
    }
}
