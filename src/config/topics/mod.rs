use std::collections::HashMap;

use group::GroupSetting;
use reset::ResetStrategy;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::io::serde::Serde;

use super::ConfigFile;

pub mod group;
pub mod reset;

pub(super) const TOPIC_CONFIG_FILE: &str = "topics.toml";

#[derive(Debug, Deserialize, Serialize)]
pub struct TopicsConfig {
    topic_configs: HashMap<String, TopicConfig>,
}

impl Default for TopicsConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl TopicsConfig {
    pub(super) fn new() -> Self {
        TopicsConfig {
            topic_configs: Default::default(),
        }
    }

    pub fn add_topic(&mut self, topic: &str, config: TopicConfig) {
        self.topic_configs.insert(topic.to_string(), config);
    }

    pub fn contains_topic(&self, topic: &str) -> bool {
        self.topic_configs.contains_key(topic)
    }

    pub fn topic(&self, topic: &str) -> Option<&TopicConfig> {
        self.topic_configs.get(topic)
    }

    pub fn topic_mut(&mut self, topic: &str) -> Option<&mut TopicConfig> {
        self.topic_configs.get_mut(topic)
    }
}

impl ConfigFile for TopicsConfig {
    fn clean(self) -> error_stack::Result<Self, crate::error::config::clusters::ConfigIoError> {
        Ok(self)
    }

    fn filename() -> &'static str {
        TOPIC_CONFIG_FILE
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicConfig {
    reset: ResetStrategy,
    group: GroupSetting,
    key_serde: Serde,
    value_serde: Serde,
}

impl TopicConfig {
    pub fn reset(&self) -> &ResetStrategy {
        &self.reset
    }

    pub fn key_serde(&self) -> &Serde {
        &self.key_serde
    }

    pub fn value_serde(&self) -> &Serde {
        &self.value_serde
    }

    pub fn reset_string(&self) -> String {
        self.reset.to_string()
    }

    pub fn group(&self) -> &GroupSetting {
        &self.group
    }

    pub fn set_key_serde(&mut self, serde: Serde) {
        self.key_serde = serde;
    }

    pub fn set_value_serde(&mut self, serde: Serde) {
        self.value_serde = serde;
    }

    pub fn group_id(&self) -> String {
        match &self.group {
            GroupSetting::Never => Uuid::new_v4().to_string(),
            GroupSetting::Group(group) => group.clone(),
        }
    }

    pub fn set_reset(&mut self, reset: ResetStrategy) {
        self.reset = reset;
    }

    pub fn set_group(&mut self, group: GroupSetting) {
        self.group = group;
    }
}
