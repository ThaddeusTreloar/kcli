use std::{fmt::Display, str::FromStr};

use clap::{builder::PossibleValue, ValueEnum};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Serde {
    AvroSchema,
    Int,
    Json,
    JsonSchema,
    #[default]
    String,
}

impl Serde {
}

impl Display for Serde {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl FromStr for Serde {
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

impl ValueEnum for Serde {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Serde::AvroSchema,
            Serde::Int,
            Serde::Json,
            Serde::JsonSchema,
            Serde::String,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Serde::AvroSchema => PossibleValue::new("avro_schema"),
            Serde::Int => PossibleValue::new("int"),
            Serde::Json => PossibleValue::new("json"),
            Serde::JsonSchema => PossibleValue::new("json_schema"),
            Serde::String => PossibleValue::new("string"),
        })
    }
}