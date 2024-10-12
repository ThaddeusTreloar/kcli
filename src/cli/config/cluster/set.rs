use clap::{Args, ValueHint};
use error_stack::{Report, ResultExt};

use crate::{
    cli::Invoke,
    config::{
        clusters::auth::{AuthType, AuthTypeNames},
        ConfigFile, Context, FromUserInputForVariant,
    },
    error::cli::config::cluster::WritableClusterError,
};

use super::util::validate_servers;

#[derive(Debug, Args)]
pub(super) struct SetCluster {
    #[arg(index = 1, help = "Logical name for the cluster.")]
    name: String,
    #[arg(short, long, value_delimiter = ',', value_hint = ValueHint::Hostname, help = "A list of bootstrap servers. Can be comma delimited or multiple invocations.")]
    bootstrap_servers: Vec<String>,
    #[arg(short, long, help = "Auth type to configure.")]
    auth: Option<AuthTypeNames>,
}

impl Invoke for SetCluster {
    type E = WritableClusterError;

    fn invoke(self, mut ctx: Context) -> error_stack::Result<(), WritableClusterError> {
        let Self {
            name,
            bootstrap_servers,
            auth,
        } = self;

        let cluster = ctx
            .clusters_mut()
            .cluster_config_mut(&name)
            .ok_or(Report::new(WritableClusterError::NotExists(name)))?;

        if !bootstrap_servers.is_empty() {
            validate_servers(&bootstrap_servers)
                .change_context(WritableClusterError::InputError("bootstrap_servers"))?;

            cluster.bootstrap_servers_mut().extend(bootstrap_servers);
        }

        if let Some(auth_type) = auth {
            let user_auth = AuthType::from_user_input_for_variant(auth_type)
                .change_context(WritableClusterError::InputError("auth"))?;

            cluster.auth_mut().replace(user_auth);
        }

        ctx.clusters()
            .write_out()
            .change_context(WritableClusterError::WriteError)
    }
}
