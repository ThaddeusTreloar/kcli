use std::collections::HashMap;

use auth::AuthType;
use error_stack::{Report, ResultExt};
use log::warn;
use serde::{Deserialize, Serialize};

use crate::{
    cli::util::get_user_choice,
    error::config::clusters::{ConfigIoError, FetchClusterError},
};

use super::ConfigFile;

pub mod auth;

pub(super) const CLUSTER_CONFIG_FILE: &str = "clusters.toml";
const SELECT_CLUSTER_PROMPT: &str = "Select cluster";

pub struct NamedCluster<'a>(pub String, pub &'a ClusterConfig);

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
        self.default.replace(cluster.to_lowercase());
    }

    pub fn unset_default(&mut self) {
        self.default.take();
    }

    pub fn list_clusters(&self) -> Vec<&String> {
        self.cluster_configs.keys().collect()
    }

    pub fn cluster_config(&self, cluster: &str) -> Option<&ClusterConfig> {
        self.cluster_configs.get(cluster.to_lowercase().as_str())
    }

    pub fn cluster_config_mut(&mut self, cluster: &str) -> Option<&mut ClusterConfig> {
        self.cluster_configs
            .get_mut(cluster.to_lowercase().as_str())
    }

    pub fn insert_cluster_config(&mut self, name: &str, cluster: ClusterConfig) {
        self.cluster_configs.insert(name.to_lowercase(), cluster);
    }

    pub fn remove_cluster_config(&mut self, name: &str) {
        self.cluster_configs.remove(name.to_lowercase().as_str());

        if let Some(default) = self.default() {
            if default == name {
                self.unset_default();
            }
        }
    }

    pub fn contains_cluster_config(&self, cluster: &str) -> bool {
        self.cluster_configs.contains_key(cluster)
    }

    pub fn cluster_config_default_or_select(
        &self,
    ) -> error_stack::Result<NamedCluster<'_>, FetchClusterError> {
        if self.default().is_some() {
            let maybe_cluster = self.get_default();

            debug_assert!(maybe_cluster.is_some());

            Ok(NamedCluster(
                self.default().unwrap().to_owned(),
                maybe_cluster.ok_or(Report::new(FetchClusterError::NotExists("".to_owned())))?,
            ))
        } else {
            warn!("No default cluster set, and no cluster provided.");

            let choices = self.list_clusters();

            if choices.is_empty() {
                Err(FetchClusterError::NoClusters)?
            }

            let choice = get_user_choice(SELECT_CLUSTER_PROMPT, choices)
                .change_context(FetchClusterError::Input)?
                .clone();

            Ok(NamedCluster(
                choice.clone(),
                self.cluster_config(&choice)
                    .expect("Failed to get cluster from list cluster choices."),
            ))
        }
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
    pub bootstrap_servers: Vec<String>,
    pub auth: Option<AuthType>,
}

impl ClusterConfig {
    pub fn new(bootstrap_servers: Vec<String>) -> Self {
        Self {
            bootstrap_servers,
            auth: None,
        }
    }

    pub fn auth(&self) -> Option<&AuthType> {
        self.auth.as_ref()
    }
}
