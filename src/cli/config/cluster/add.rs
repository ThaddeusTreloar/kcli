use clap::{Args, ValueHint};
use error_stack::{Report, ResultExt};

use crate::{
    cli::{
        util::{get_user_input_confirmation, get_user_input_vec},
        Invoke,
    },
    config::{
        clusters::{
            auth::{AuthType, AuthTypeNames},
            ClusterConfig,
        },
        ConfigFile, Context, FromUserInputForVariant,
    },
    error::cli::config::cluster::{AddClusterError, WritableClusterError},
};

use super::util::validate_servers;

const BOOTSTRAP_SERVERS_PROMPT: &str = "Add bootstrap server (Press enter to finish):";
const SET_DEFAULT_PROMPT: &str = "No default set, set this cluster as default?";

#[derive(Debug, Args)]
pub(super) struct AddCluster {
    #[arg(index = 1, help = "Logical name for the cluster.")]
    name: String,
    #[arg(short, long, value_delimiter = ',', value_hint = ValueHint::Hostname, help = "A list of bootstrap servers. Can be comma delimited or multiple invocations.")]
    bootstrap_servers: Vec<String>,
    #[arg(short, long, help = "Auth type to configure.")]
    auth: Option<AuthTypeNames>,
    #[arg(long, help = "Refuse user input.")]
    no_input: bool,
}

impl Invoke for AddCluster {
    type E = AddClusterError;

    fn invoke(self, mut ctx: Context) -> error_stack::Result<(), AddClusterError> {
        let Self {
            name,
            mut bootstrap_servers,
            no_input,
            auth,
        } = self;

        if ctx.clusters().contains_cluster_config(&name)
            && !get_user_input_confirmation("Cluster config exists, do you want to replace it?")
                .change_context(AddClusterError::InputError("replace cluster"))?
        {
            return Err(Report::new(AddClusterError::AlreadyExists(name)));
        }

        if bootstrap_servers.is_empty() {
            if no_input {
                return Err(Report::new(AddClusterError::MissingBootstrapServers));
            } else {
                bootstrap_servers.extend(
                    get_user_input_vec(BOOTSTRAP_SERVERS_PROMPT)
                        .change_context(AddClusterError::InputError("bootstrap_servers"))?,
                );
            }
        }

        validate_servers(&bootstrap_servers)
            .change_context(AddClusterError::InputError("bootstrap_servers"))?;

        let mut cluster = ClusterConfig::new(bootstrap_servers);

        if let Some(auth_type) = auth {
            let user_auth = AuthType::from_user_input_for_variant(auth_type)
                .change_context(AddClusterError::InputError("auth"))?;

            cluster.auth_mut().replace(user_auth);
        }

        ctx.clusters_mut().insert_cluster_config(&name, cluster);

        if ctx.clusters().default().is_none() {
            let confirm = get_user_input_confirmation(SET_DEFAULT_PROMPT)
                .change_context(AddClusterError::InputError("confirmation"))?;

            if confirm {
                ctx.clusters_mut().set_default(&name);
            }
        }

        ctx.clusters()
            .write_out()
            .change_context(AddClusterError::WriteError)
    }
}
