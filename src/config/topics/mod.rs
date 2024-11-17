use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::io::serde::Serde;

use super::{profiles::ProfileConfig, ConfigFile};

pub(super) const TOPIC_CONFIG_FILE: &str = "topics.toml";

#[derive(Debug, Deserialize, Serialize)]
pub struct TopicsConfig {
    pub topic_configs: HashMap<String, TopicConfig>,
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
    pub default_profile: Option<String>,
    pub key_serde: Serde,
    pub value_serde: Serde,
}

impl TopicConfig {
    pub fn default_profile(&self) -> Option<&String> {
        self.default_profile.as_ref()
    }
}
