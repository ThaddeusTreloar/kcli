use std::{error::Error, fs::{create_dir, exists}, path::PathBuf};
use dirs::home_dir;
use error_stack::ResultExt;

use clusters::ClustersConfig;
use log::info;

use crate::error::config::{InitContextError, PathError};

pub mod clusters;

const CONFIG_HOME: &str = ".kafkacli";

#[derive(Debug)]
pub struct Context {
    clusters: ClustersConfig,
}

impl Context {
    pub fn init() -> error_stack::Result<Self, InitContextError> {
        let home_path = home_dir()
            .ok_or(InitContextError::CreateDirectory("~/.kafkacli".to_owned()))
            .attach_printable("Failed to get home directory.")?;
    
        let mut config_home_path = home_path.clone();

        config_home_path.push(CONFIG_HOME);
        
        let resolved_config_home_path = config_home_path.as_path();
        
        if !exists(resolved_config_home_path)
            .change_context(InitContextError::CheckExistence(resolved_config_home_path.display().to_string()))?
        {
            info!("Config home not found, creating at: {}", resolved_config_home_path.display());
        
            create_dir(resolved_config_home_path)
                .change_context(InitContextError::CreateDirectory(resolved_config_home_path.display().to_string()))?;
        }
        
        let clusters = ClustersConfig::read_in()
            .change_context(InitContextError::LoadConfig("clusters"))?;

        Ok(Self {
            clusters
        })
    }

    fn get_path_for_child(child_path: &str) -> error_stack::Result<PathBuf, PathError> {
        let mut path = home_dir()
            .ok_or(PathError::GetPath("~/.kafkacli".to_owned()))
            .attach_printable("Failed to get home directory.")?;

        path.push(CONFIG_HOME);
        path.push(child_path);

        Ok(path)
    }

    pub fn clusters(&self) -> &ClustersConfig {
        &self.clusters
    }
    pub fn clusters_mut(&mut self) -> &mut ClustersConfig {
        &mut self.clusters
    }
}

pub fn init_config() -> error_stack::Result<(), InitContextError> {

    Ok(())
}

pub trait FromUserInput {
    type E: Error;

    fn from_user_input() -> error_stack::Result<Self, Self::E> where Self: Sized;
}

pub trait FromUserInputForVariant {
    type E: Error;
    type V;

    fn from_user_input_for_variant(variante: Self::V) -> error_stack::Result<Self, Self::E> where Self: Sized;
}