use std::{fs::{create_dir, exists, File}, io::Write, path::Path};
use dirs::home_dir;
use error_stack::ResultExt;

use clusters::ClustersConfig;
use log::info;

use crate::error::config::InitConfigError;

pub (crate) mod clusters;

const CONFIG_HOME: &str = ".kafkacli";
const CLUSTER_CONFIG: &str = ".kafkacli/clusters.toml";

pub(crate) fn init_config() -> error_stack::Result<(), InitConfigError> {
    let mut config_home_path = home_dir()
        .ok_or(InitConfigError::CreateDirectory("~/.kafkacli".to_owned()))
        .attach_printable("Failed to get home directory.")?;

    config_home_path.push(CONFIG_HOME);

    let resolved_config_home_path = config_home_path.as_path();

    if !exists(resolved_config_home_path)
        .change_context(InitConfigError::CheckExistence(resolved_config_home_path.display().to_string()))?
    {
        info!("Config home not found, creating at: {}", resolved_config_home_path.display());

        create_dir(resolved_config_home_path)
            .change_context(InitConfigError::CreateDirectory(resolved_config_home_path.display().to_string()))?;
    }

    let mut cluster_config_path = home_dir()
        .ok_or(InitConfigError::CreateDirectory("~/.kafkacli".to_owned()))
        .attach_printable("Failed to get home directory.")?;

    cluster_config_path.push(CLUSTER_CONFIG);

    let resolved_cluster_config_path = cluster_config_path.as_path();

    if !exists(resolved_cluster_config_path)
        .change_context(InitConfigError::CheckExistence(resolved_cluster_config_path.display().to_string()))? 
    {
        info!("Cluster config not found, creating at: {}", resolved_cluster_config_path.display());

        let mut file = File::create(resolved_cluster_config_path)
            .change_context(InitConfigError::CreateFile(resolved_cluster_config_path.display().to_string()))?;

        let config = ClustersConfig::new();

        let toml_content = toml::to_string(&config)
            .change_context(InitConfigError::WriteFile(resolved_cluster_config_path.display().to_string()))?;

        file.write_all(toml_content.as_bytes())
            .change_context(InitConfigError::WriteFile(resolved_cluster_config_path.display().to_string()))?;
    }

    Ok(())
}