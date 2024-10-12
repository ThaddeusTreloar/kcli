use std::{fmt::Display, str::FromStr};

use clap::{builder::PossibleValue, ValueEnum};
use sasl_plain::SaslPlain;
use sasl_ssl::SaslSsl;
use serde::{Deserialize, Serialize};

use crate::{config::{FromUserInput, FromUserInputForVariant}, error::cli::util::UserInputError};

pub mod sasl_plain;
pub mod sasl_ssl;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum AuthType {
    #[default]
    Plain,
    SaslPlain(SaslPlain),
    SaslSsl(SaslSsl),
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum AuthTypeNames {
    #[default]
    Plain,
    SaslPlain,
    SaslSsl,
}

impl FromUserInputForVariant for AuthType {
    type E = UserInputError;
    type V = AuthTypeNames;

    fn from_user_input_for_variant(variant: Self::V) -> error_stack::Result<Self, Self::E> where Self: Sized {
        match variant {
            AuthTypeNames::Plain => Ok(Self::Plain),
            AuthTypeNames::SaslPlain => SaslPlain::from_user_input().map(AuthType::SaslPlain),
            AuthTypeNames::SaslSsl => todo!("Todo"),
        }
    }
}

impl Display for AuthTypeNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl FromStr for AuthTypeNames {
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

impl ValueEnum for AuthTypeNames {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            AuthTypeNames::Plain,
            AuthTypeNames::SaslPlain,
            AuthTypeNames::SaslSsl,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            AuthTypeNames::Plain => PossibleValue::new("plain"),
            AuthTypeNames::SaslPlain => PossibleValue::new("sasl_plain"),
            AuthTypeNames::SaslSsl => PossibleValue::new("sasl_ssl"),
        })
    }
}