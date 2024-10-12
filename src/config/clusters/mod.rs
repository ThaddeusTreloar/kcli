use std::{collections::HashMap, fs::{exists, File}, io::{Read, Write}};

use auth::AuthType;
use error_stack::ResultExt;
use log::{info, warn};
use serde::{Deserialize, Serialize};

use crate::error::config::{clusters::ClusterConfigIoError, InitContextError};

use super::Context;

pub mod auth;

pub (super) const CLUSTER_CONFIG: &str = "clusters.toml";

#[derive(Debug, Deserialize, Serialize)]
pub struct ClustersConfig {
    default: Option<String>,
    cluster_configs: HashMap<String, ClusterConfig>
}

impl ClustersConfig {
    pub (super) fn new() -> Self {
        ClustersConfig { default:None, cluster_configs: HashMap::new() }
    }

    pub (super) fn create_if_not_exists() -> error_stack::Result<Self, InitContextError> {
        let cluster_config_path = Context::get_path_for_child(CLUSTER_CONFIG)
            .change_context(InitContextError::CheckExistence(CLUSTER_CONFIG.to_owned()))?;
        
        let resolved_cluster_config_path = cluster_config_path.as_path();
        
        if !exists(resolved_cluster_config_path)
            .change_context(InitContextError::CheckExistence(resolved_cluster_config_path.display().to_string()))? 
        {
            info!("Cluster config not found, creating at: {}", resolved_cluster_config_path.display());
        
            let mut file = File::create(resolved_cluster_config_path)
                .change_context(InitContextError::CreateFile(resolved_cluster_config_path.display().to_string()))?;
        
            let config = ClustersConfig::new();
        
            let toml_content = toml::to_string(&config)
                .change_context(InitContextError::WriteFile(resolved_cluster_config_path.display().to_string()))?;
        
            file.write_all(toml_content.as_bytes())
                .change_context(InitContextError::WriteFile(resolved_cluster_config_path.display().to_string()))?;

            Ok(config)
        } else {
            ClustersConfig::read_in()
                .change_context(InitContextError::ReadFile(resolved_cluster_config_path.display().to_string()))
        }
    }

    pub fn clean(mut self) -> error_stack::Result<Self, ClusterConfigIoError>{
        if self.default().is_some() && self.get_default().is_none() {
            warn!("Default cluster '{}' does not exists. Unsetting default", self.default().expect("Unexpected error."));

            self.unset_default();
            self.write_out()
                .change_context(ClusterConfigIoError::Write("during config clean.".to_string()))
                .map(|_| self)
        } else {
            Ok(self)
        }
    }

    pub fn read_in() -> error_stack::Result<Self, ClusterConfigIoError> {
        let config_path = Context::get_path_for_child(CLUSTER_CONFIG)
            .change_context(ClusterConfigIoError::Read(CLUSTER_CONFIG.to_owned()))?;

        let resolved_config_path = config_path.as_path();

        let mut config_file = File::open(resolved_config_path)
            .change_context(ClusterConfigIoError::Read(resolved_config_path.display().to_string()))?;

        let mut raw_config = String::new();

        config_file.read_to_string(&mut raw_config)
            .change_context(ClusterConfigIoError::Read(resolved_config_path.display().to_string()))?;

        let config: Self = toml::from_str(&raw_config)
            .change_context(ClusterConfigIoError::Parse)?;

        config.clean()
    }

    pub fn write_out(&self) -> error_stack::Result<(), ClusterConfigIoError> {
        let config_path = Context::get_path_for_child(CLUSTER_CONFIG)
            .change_context(ClusterConfigIoError::Read(CLUSTER_CONFIG.to_owned()))?;

        let resolved_config_path = config_path.as_path();

        let mut config_file = File::create(resolved_config_path)
            .change_context(ClusterConfigIoError::Read(resolved_config_path.display().to_string()))?;

        let config_str = toml::to_string_pretty(self)
            .change_context(ClusterConfigIoError::Parse)?;

        config_file.write_all(config_str.as_bytes())
            .change_context(ClusterConfigIoError::Write(resolved_config_path.display().to_string()))
    }

    pub fn default(&self) -> Option<&String> {
        self.default.as_ref()
    }

    pub fn get_default(&self) -> Option<&ClusterConfig> {
        self.default().and_then(|c|self.cluster_config(c))
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

#[derive(Debug, Deserialize, Serialize)]
pub struct ClusterConfig {
    bootstrap_servers: Vec<String>,
    auth: Option<AuthType>,
}

impl ClusterConfig {
    pub fn new(bootstrap_servers: Vec<String>) -> Self {
        Self {
            bootstrap_servers,
            auth: None,
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
}

