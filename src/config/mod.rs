use dirs::home_dir;
use error_stack::ResultExt;
use std::{
    error::Error,
    fs::{create_dir, exists, File},
    io::{Read, Write},
    path::PathBuf,
};

use clusters::ClustersConfig;
use log::info;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use topics::TopicsConfig;

use crate::error::config::{clusters::ConfigIoError, InitContextError, PathError};

pub mod clusters;
pub mod topics;

const CONFIG_HOME: &str = ".kcli";

#[derive(Debug)]
pub struct Context {
    clusters: ClustersConfig,
    topics: TopicsConfig,
}

impl Context {
    pub fn write_out(&self) -> error_stack::Result<(), ConfigIoError> {
        self.clusters.write_out()?;
        self.topics.write_out()?;

        Ok(())
    }

    pub fn init() -> error_stack::Result<Self, InitContextError> {
        let home_path = home_dir()
            .ok_or(InitContextError::CreateDirectory("~/.kafkacli".to_owned()))
            .attach_printable("Failed to get home directory.")?;

        let mut config_home_path = home_path.clone();

        config_home_path.push(CONFIG_HOME);

        let resolved_config_home_path = config_home_path.as_path();

        if !exists(resolved_config_home_path).change_context(InitContextError::CheckExistence(
            resolved_config_home_path.display().to_string(),
        ))? {
            info!(
                "Config home not found, creating at: {}",
                resolved_config_home_path.display()
            );

            create_dir(resolved_config_home_path).change_context(
                InitContextError::CreateDirectory(resolved_config_home_path.display().to_string()),
            )?;
        }

        let clusters = ClustersConfig::create_if_not_exists()
            .change_context(InitContextError::LoadConfig("clusters"))?;

        let topics = TopicsConfig::create_if_not_exists()
            .change_context(InitContextError::LoadConfig("topics"))?;

        Ok(Self { clusters, topics })
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

    pub fn topics(&self) -> &TopicsConfig {
        &self.topics
    }
    pub fn topics_mut(&mut self) -> &mut TopicsConfig {
        &mut self.topics
    }
}

pub fn init_config() -> error_stack::Result<(), InitContextError> {
    Ok(())
}

pub trait FromUserInput {
    type E: Error;

    fn from_user_input() -> error_stack::Result<Self, Self::E>
    where
        Self: Sized;
}

pub trait FromUserInputForVariant {
    type E: Error;
    type V;

    fn from_user_input_for_variant(variante: Self::V) -> error_stack::Result<Self, Self::E>
    where
        Self: Sized;
}

pub trait ConfigFile: Default + DeserializeOwned + Serialize {
    fn clean(self) -> error_stack::Result<Self, ConfigIoError>;

    fn create_if_not_exists() -> error_stack::Result<Self, InitContextError> {
        let config_path = Context::get_path_for_child(Self::filename()).change_context(
            InitContextError::CheckExistence(Self::filename().to_owned()),
        )?;

        let resolved_config_path = config_path.as_path();

        if !exists(resolved_config_path).change_context(InitContextError::CheckExistence(
            resolved_config_path.display().to_string(),
        ))? {
            info!(
                "Topic config not found, creating at: {}",
                resolved_config_path.display()
            );

            let mut file = File::create(resolved_config_path).change_context(
                InitContextError::CreateFile(resolved_config_path.display().to_string()),
            )?;

            let config = Self::default();

            let toml_content = toml::to_string(&config).change_context(
                InitContextError::WriteFile(resolved_config_path.display().to_string()),
            )?;

            file.write_all(toml_content.as_bytes())
                .change_context(InitContextError::WriteFile(
                    resolved_config_path.display().to_string(),
                ))?;

            Ok(config)
        } else {
            Self::read_in().change_context(InitContextError::ReadFile(
                resolved_config_path.display().to_string(),
            ))
        }
    }

    fn filename() -> &'static str;

    fn read_in() -> error_stack::Result<Self, ConfigIoError> {
        let config_path = Context::get_path_for_child(Self::filename())
            .change_context(ConfigIoError::Read(Self::filename().to_owned()))?;

        let resolved_config_path = config_path.as_path();

        let mut config_file = File::open(resolved_config_path).change_context(
            ConfigIoError::Read(resolved_config_path.display().to_string()),
        )?;

        let mut raw_config = String::new();

        config_file
            .read_to_string(&mut raw_config)
            .change_context(ConfigIoError::Read(
                resolved_config_path.display().to_string(),
            ))?;

        let config: Self = toml::from_str(&raw_config).change_context(ConfigIoError::Parse)?;

        config.clean()
    }

    fn write_out(&self) -> error_stack::Result<(), ConfigIoError> {
        let config_path = Context::get_path_for_child(Self::filename())
            .change_context(ConfigIoError::Read(Self::filename().to_owned()))?;

        let resolved_config_path = config_path.as_path();

        let mut config_file = File::create(resolved_config_path).change_context(
            ConfigIoError::Read(resolved_config_path.display().to_string()),
        )?;

        let config_str = toml::to_string_pretty(self).change_context(ConfigIoError::Parse)?;

        config_file
            .write_all(config_str.as_bytes())
            .change_context(ConfigIoError::Write(
                resolved_config_path.display().to_string(),
            ))
    }
}
