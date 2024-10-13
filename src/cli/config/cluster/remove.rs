use clap::Args;
use error_stack::{Report, ResultExt};

use crate::{
    cli::{
        util::{get_user_choice, get_user_input_confirmation},
        Invoke,
    },
    config::Context,
    error::cli::config::cluster::WritableClusterError,
};

const CHOOSE_CLUSTER_PROMPT: &str = "Select cluster to remove:";
const REMOVE_DEFAULT_PROMPT: &str = "This cluster is set as default, remove default cluster?";

#[derive(Debug, Args)]
pub(super) struct RemoveCluster {
    #[arg(index = 1, help = "Logical name for the cluster.")]
    name: Option<String>,
}

impl Invoke for RemoveCluster {
    type E = WritableClusterError;

    fn invoke(self, ctx: &mut Context) -> error_stack::Result<(), WritableClusterError> {
        let Self { name } = self;

        let name = match name {
            Some(name) => name,
            None => {
                let choices = ctx.clusters().list_clusters();

                if choices.is_empty() {
                    Err(Report::new(WritableClusterError::NotExists(
                        "No clusters to delete".to_owned(),
                    )))?
                }

                get_user_choice(CHOOSE_CLUSTER_PROMPT, choices)
                    .change_context(WritableClusterError::InputError("choose cluster"))?
                    .to_owned()
            }
        };

        if !ctx.clusters().contains_cluster_config(&name) {
            Err(Report::new(WritableClusterError::NotExists(name)))
        } else if !get_user_input_confirmation(&format!(
            "Are you sure you want to remove '{}'?",
            name
        ))
        .change_context(WritableClusterError::InputError("confirmation"))?
        {
            Ok(())
        } else {
            match ctx.clusters().default() {
                Some(cluster) if cluster == &name => {
                    let confirm = get_user_input_confirmation(REMOVE_DEFAULT_PROMPT)
                        .change_context(WritableClusterError::InputError("confirmation"))?;

                    if !confirm {
                        return Ok(());
                    }
                }
                _ => (),
            }

            ctx.clusters_mut().remove_cluster_config(&name);

            Ok(())
        }
    }
}
