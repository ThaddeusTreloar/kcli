use clap::Args;
use error_stack::Report;

use crate::{
    cli::{GlobalArgs, Invoke},
    config::Context,
    error::cli::config::cluster::WritableClusterError,
};

#[derive(Debug, Args)]
pub(super) struct DefaultCluster {
    #[arg(index = 1, help = "Logical name for the cluster.")]
    name: Option<String>,
}

impl Invoke for DefaultCluster {
    type E = WritableClusterError;

    fn invoke(
        self,
        ctx: &mut Context,
        global_args: &GlobalArgs,
    ) -> error_stack::Result<(), WritableClusterError> {
        let Self { name } = self;

        let name = match name {
            Some(name) => name,
            None => {
                match ctx.clusters.default() {
                    Some(name) => println!("Default cluster: {}", name),
                    None => println!("No default cluster set."),
                }

                return Ok(());
            }
        };

        if !ctx.clusters.contains_cluster_config(&name) {
            Err(Report::new(WritableClusterError::NotExists(name)))
        } else {
            ctx.clusters.set_default(&name);

            println!("Set '{}' as default cluster.", name);

            Ok(())
        }
    }
}
