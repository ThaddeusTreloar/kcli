use std::{fmt::Display, str::FromStr};

use clap::{builder::PossibleValue, ValueEnum};
use error_stack::{Report, ResultExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::io::SerdeError;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Serde {
    AvroSchema,
    Int,
    Bytes,
    Json,
    JsonSchema,
    #[default]
    String,
}

impl Serde {
    pub fn deserialise_into_string(
        &self,
        bytes: Vec<u8>,
    ) -> error_stack::Result<String, SerdeError> {
        match self {
            Serde::String => String::from_utf8(bytes).change_context(SerdeError::Deserialise),
            Serde::Bytes => Ok(format!("{:?}", bytes)),
            Serde::Int => {
                let buff: [u8; 4] = <Vec<u8> as TryInto<[u8; 4]>>::try_into(bytes)
                    .map_err(|_| Report::new(SerdeError::Deserialise))
                    .attach_printable("Failed to convert input bytes into [u8; 4]")?;

                Ok(i32::from_be_bytes(buff).to_string())
            }
            Serde::Json => serde_json::to_string_pretty(
                &serde_json::from_slice::<Value>(&bytes).change_context(SerdeError::Deserialise)?,
            )
            .change_context(SerdeError::Deserialise),
            _ => todo!("Not yet implemented"),
        }
    }
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
            Serde::Bytes,
            Serde::Int,
            Serde::Json,
            Serde::JsonSchema,
            Serde::String,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Serde::AvroSchema => PossibleValue::new("avro_schema"),
            Serde::Bytes => PossibleValue::new("bytes"),
            Serde::Int => PossibleValue::new("int"),
            Serde::Json => PossibleValue::new("json"),
            Serde::JsonSchema => PossibleValue::new("json_schema"),
            Serde::String => PossibleValue::new("string"),
        })
    }
}
