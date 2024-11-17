use std::collections::HashMap;

use group::GroupSetting;
use reset::ResetStrategy;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::ConfigFile;

pub mod group;
pub mod reset;

pub(super) const PROFILE_CONFIG_FILE: &str = "profiles.toml";
const SELECT_PROFILE_PROMPT: &str = "Select profile";

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ProfilesConfig {
    pub default_profile: Option<String>,
    pub profile_configs: HashMap<String, ProfileConfig>,
}

impl ProfilesConfig {
    pub(super) fn new() -> Self {
        Default::default()
    }

    pub fn new_default(&mut self, profile: &str) -> &ProfileConfig {
        self.default_profile.replace(profile.to_owned());

        if !self.contains_profile(profile) {
            self.add_profile(profile, ProfileConfig::default());
        }

        self.profile(profile).unwrap()
    }

    pub fn add_profile(&mut self, profile: &str, config: ProfileConfig) {
        self.profile_configs.insert(profile.to_string(), config);
    }

    pub fn contains_profile(&self, profile: &str) -> bool {
        self.profile_configs.contains_key(profile)
    }

    pub fn default_profile(&self) -> Option<&ProfileConfig> {
        self.default_profile.as_ref().and_then(|p| self.profile(p))
    }

    pub fn profile(&self, profile: &str) -> Option<&ProfileConfig> {
        self.profile_configs.get(profile)
    }

    pub fn profile_mut(&mut self, profile: &str) -> Option<&mut ProfileConfig> {
        self.profile_configs.get_mut(profile)
    }
}

impl ConfigFile for ProfilesConfig {
    fn clean(self) -> error_stack::Result<Self, crate::error::config::clusters::ConfigIoError> {
        Ok(self)
    }

    fn filename() -> &'static str {
        PROFILE_CONFIG_FILE
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ProfileConfig {
    pub reset: ResetStrategy,
    pub group: GroupSetting,
}

impl ProfileConfig {
    pub fn group_id(&self) -> String {
        match &self.group {
            GroupSetting::Never => Uuid::new_v4().to_string(),
            GroupSetting::Group(group) => group.clone(),
        }
    }

    pub fn with_maybe_reset(mut self, reset: Option<ResetStrategy>) -> Self {
        if let Some(reset) = reset {
            self.reset = reset;
        }

        self
    }

    pub fn with_maybe_group(mut self, group: Option<String>) -> Self {
        if let Some(group) = group {
            self.group = GroupSetting::Group(group);
        }

        self
    }
}
