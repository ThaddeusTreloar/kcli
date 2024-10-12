use std::{collections::HashMap, io::Read};

use auth::AuthType;
use error_stack::ResultExt;
use log::warn;
use serde::{Deserialize, Serialize};

use crate::error::config::clusters::ConfigIoError;

use super::ConfigFile;

pub mod auth;

pub(super) const CLUSTER_CONFIG_FILE: &str = "clusters.toml";

#[derive(Debug, Deserialize, Serialize)]
pub struct ClustersConfig {
    default: Option<String>,
    cluster_configs: HashMap<String, ClusterConfig>,
}

impl Default for ClustersConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl ClustersConfig {
    pub(super) fn new() -> Self {
        ClustersConfig {
            default: None,
            cluster_configs: HashMap::new(),
        }
    }

    pub fn default(&self) -> Option<&String> {
        self.default.as_ref()
    }

    pub fn get_default(&self) -> Option<&ClusterConfig> {
        self.default().and_then(|c| self.cluster_config(c))
    }

    pub fn get_default_mut(&mut self) -> Option<&mut ClusterConfig> {
        let default = match self.default() {
            Some(default) => default.clone(),
            None => None?,
        };

        self.cluster_config_mut(&default)
    }

    pub fn set_default(&mut self, cluster: &str) {
        self.default.replace(cluster.to_owned());
    }

    pub fn unset_default(&mut self) {
        self.default.take();
    }

    pub fn list_clusters(&self) -> Vec<&String> {
        self.cluster_configs.keys().collect()
    }

    pub fn cluster_config(&self, cluster: &str) -> Option<&ClusterConfig> {
        self.cluster_configs.get(cluster)
    }

    pub fn cluster_config_mut(&mut self, cluster: &str) -> Option<&mut ClusterConfig> {
        self.cluster_configs.get_mut(cluster)
    }

    pub fn insert_cluster_config(&mut self, name: &str, cluster: ClusterConfig) {
        self.cluster_configs.insert(name.to_owned(), cluster);
    }

    pub fn remove_cluster_config(&mut self, name: &str) {
        self.cluster_configs.remove(name);

        if let Some(default) = self.default() {
            if default == name {
                self.unset_default();
            }
        }
    }

    pub fn contains_cluster_config(&self, cluster: &str) -> bool {
        self.cluster_configs.contains_key(cluster)
    }
}

impl ConfigFile for ClustersConfig {
    fn filename() -> &'static str {
        CLUSTER_CONFIG_FILE
    }

    fn clean(mut self) -> error_stack::Result<Self, ConfigIoError> {
        if self.default().is_some() && self.get_default().is_none() {
            warn!(
                "Default cluster '{}' does not exists. Unsetting default",
                self.default().expect("Unexpected error.")
            );

            self.unset_default();
            self.write_out()
                .change_context(ConfigIoError::Write("during config clean.".to_string()))
                .map(|_| self)
        } else {
            Ok(self)
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClusterConfig {
    bootstrap_servers: Vec<String>,
    auth: Option<AuthType>,
    topics: Vec<String>,
}

impl ClusterConfig {
    pub fn new(bootstrap_servers: Vec<String>) -> Self {
        Self {
            bootstrap_servers,
            auth: None,
            topics: Vec::new(),
        }
    }

    pub fn bootstrap_servers(&self) -> &[String] {
        &self.bootstrap_servers
    }

    pub fn bootstrap_servers_mut(&mut self) -> &mut Vec<String> {
        &mut self.bootstrap_servers
    }

    pub fn auth(&self) -> Option<&AuthType> {
        self.auth.as_ref()
    }

    pub fn auth_mut(&mut self) -> &mut Option<AuthType> {
        &mut self.auth
    }

    pub fn topics(&self) -> &Vec<String> {
        &self.topics
    }

    pub fn contains_topic(&self, topic: &String) -> bool {
        self.topics.contains(topic)
    }

    pub fn add_topic(&mut self, topic: &str) {
        self.topics.push(topic.to_owned());
    }

    pub fn remove_topic(&mut self, topic: &str) {
        self.topics.push(topic.to_owned());
    }
}
