use std::{fs::File, net::{SocketAddr, ToSocketAddrs}};

use clap::{Args, ValueHint};
use dirs::home_dir;
use error_stack::{Report, ResultExt};

use crate::{cli::util::{get_user_input_choice, get_user_input_vec}, config::clusters::{ClusterConfig, ClustersConfig}, error::cli::config::cluster::CreateClusterError};

const BOOTSTRAP_SERVERS_PROMPT: &str = "Add bootstrap server (Press enter to finish): ";

#[derive(Debug, Args)]
pub (super) struct CreateCluster {
    #[arg(short, long, help = "Logical name for the cluster.")]
    name: String,
    #[arg(short, long, value_delimiter = ',', value_hint = ValueHint::Hostname, help = "A list of bootstrap servers. Can be comma delimited or multiple invocations.")]
    bootstrap_servers: Vec<String>,
    #[arg(long, help = "Refuse user input.")]
    no_input: bool,
}

impl CreateCluster {
    pub (super) fn execute(self) -> error_stack::Result<(), CreateClusterError> {
        let Self { name, mut bootstrap_servers, no_input } = self;

        if bootstrap_servers.is_empty() {
            if no_input {
                return Err(Report::new(CreateClusterError::MissingBootstrapServers));
            } else {
                bootstrap_servers.extend(
                    get_user_input_vec(BOOTSTRAP_SERVERS_PROMPT)
                        .change_context(CreateClusterError::InputError("bootstrap_servers"))?
                );
            }
        }

        // Validate host addresses
        let unvalidated_bootstrap_servers: Result<Vec<_>, std::io::Error> = bootstrap_servers
            .iter()
            .map(|address|address.to_socket_addrs())
            .collect();

        unvalidated_bootstrap_servers
            .change_context(CreateClusterError::InputError("bootstrap_servers"))
            .attach_printable_lazy(|| {
                format!("Failed to parse socket addresses for bootstrap_servers: {:?}", bootstrap_servers)
            })?;

        let config = ClusterConfig::new(bootstrap_servers);

        let mut existing_configs = ClustersConfig::read_in()
            .change_context(CreateClusterError::WriteError)?;

        if existing_configs.contains_cluster_config(&name) 
            && !get_user_input_choice("Cluster config exists, do you want to replace it?")
                    .change_context(CreateClusterError::InputError("replace cluster"))? 
        {
            return Err(Report::new(CreateClusterError::AlreadyExists(name)));
        }

        existing_configs.insert_cluster_config(&name, config);

        existing_configs.write_out()
            .change_context(CreateClusterError::WriteError)
    }
}