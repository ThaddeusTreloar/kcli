use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum GroupSetting {
    #[default]
    Never,
    Group(String),
}

impl Display for GroupSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Never => f.write_str("Never"),
            Self::Group(group) => f.write_str(&format!("Group({})", group)),
        }
    }
}

impl From<Option<String>> for GroupSetting {
    fn from(value: Option<String>) -> Self {
        match value {
            None => Self::Never,
            Some(group) => Self::Group(group),
        }
    }
}
