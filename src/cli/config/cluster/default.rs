use clap::Args;
use error_stack::{Report, ResultExt};

use crate::{
    cli::Invoke,
    config::{ConfigFile, Context},
    error::cli::config::cluster::WritableClusterError,
};

#[derive(Debug, Args)]
pub(super) struct DefaultCluster {
    #[arg(index = 1, help = "Logical name for the cluster.")]
    name: Option<String>,
}

impl Invoke for DefaultCluster {
    type E = WritableClusterError;

    fn invoke(self, mut ctx: &mut Context) -> error_stack::Result<(), WritableClusterError> {
        let Self { name } = self;

        let name = match name {
            Some(name) => name,
            None => {
                match ctx.clusters().default() {
                    Some(name) => println!("Default cluster: {}", name),
                    None => println!("No default cluster set."),
                }

                return Ok(());
            }
        };

        if !ctx.clusters().contains_cluster_config(&name) {
            Err(Report::new(WritableClusterError::NotExists(name)))
        } else {
            ctx.clusters_mut().set_default(&name);

            println!("Set '{}' as default cluster.", name);

            Ok(())
        }
    }
}
