use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum GroupSetting {
    #[default]
    Never,
    Group(String),
}

impl From<Option<String>> for GroupSetting {
    fn from(value: Option<String>) -> Self {
        match value {
            None => Self::Never,
            Some(group) => Self::Group(group),
        }
    }
}
