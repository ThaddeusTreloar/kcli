use std::{collections::HashMap, fs::File, io::{Read, Write}};

use dirs::home_dir;
use error_stack::ResultExt;
use serde::{Deserialize, Serialize};

use crate::error::config::clusters::ClusterConfigIoError;

use super::CLUSTER_CONFIG;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClustersConfig {
    default: Option<String>,
    cluster_configs: HashMap<String, ClusterConfig>
}

impl ClustersConfig {
    pub (super) fn new() -> Self {
        ClustersConfig { default:None, cluster_configs: HashMap::new() }
    }

    pub (crate) fn read_in() -> error_stack::Result<Self, ClusterConfigIoError> {
        let mut config_path = home_dir().ok_or(ClusterConfigIoError::ResolutionError("~/".to_owned()))?;

        config_path.push(CLUSTER_CONFIG);

        let resolved_config_path = config_path.as_path();

        let mut config_file = File::open(resolved_config_path)
            .change_context(ClusterConfigIoError::ReadError(resolved_config_path.display().to_string()))?;

        let mut raw_config = String::new();

        config_file.read_to_string(&mut raw_config);

        let config = toml::from_str(&raw_config)
            .change_context(ClusterConfigIoError::ParseError)?;

        Ok(config)
    }

    pub (crate) fn write_out(&self) -> error_stack::Result<(), ClusterConfigIoError> {
        let mut config_path = home_dir().ok_or(ClusterConfigIoError::ResolutionError("~/".to_owned()))?;

        config_path.push(CLUSTER_CONFIG);

        let resolved_config_path = config_path.as_path();

        let mut config_file = File::create(resolved_config_path)
            .change_context(ClusterConfigIoError::ReadError(resolved_config_path.display().to_string()))?;

        let config_str = toml::to_string_pretty(self)
            .change_context(ClusterConfigIoError::ParseError)?;

        config_file.write_all(config_str.as_bytes())
            .change_context(ClusterConfigIoError::WriteError(resolved_config_path.display().to_string()))
    }

    pub (crate) fn default(&self) -> Option<&String> {
        self.default.as_ref()
    }

    pub (crate) fn list_clusters(&self) -> Vec<&String> {
        self.cluster_configs.keys().collect()
    }

    pub (crate) fn get_cluster_config(&self, cluster: &str) -> Option<&ClusterConfig> {
        self.cluster_configs.get(cluster)
    }

    pub (crate) fn insert_cluster_config(&mut self, name: &str, cluster: ClusterConfig) {
        self.cluster_configs.insert(name.to_owned(), cluster);
    }

    pub (crate) fn contains_cluster_config(&self, cluster: &str) -> bool {
        self.cluster_configs.contains_key(cluster)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClusterConfig {
    bootstrap_servers: Vec<String>,
}

impl ClusterConfig {
    pub (crate) fn new(bootstrap_servers: Vec<String>) -> Self {
        Self {
            bootstrap_servers
        }
    }
}

