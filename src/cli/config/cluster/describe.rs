use clap::Args;
use error_stack::{Report, ResultExt};

use crate::{
    cli::{GlobalArgs, Invoke},
    config::Context,
    error::cli::config::cluster::ReadOnlyClusterError,
};

#[derive(Debug, Args)]
pub(super) struct DescribeCluster {
    #[arg(index = 1, help = "Logical name for the cluster.")]
    name: Option<String>,
}

impl Invoke for DescribeCluster {
    type E = ReadOnlyClusterError;

    fn invoke(
        self,
        ctx: &mut Context,
        global_args: &GlobalArgs,
    ) -> error_stack::Result<(), ReadOnlyClusterError> {
        let Self { name } = self;

        let name = match name {
            Some(name) => name,
            None => ctx
                .clusters
                .default()
                .ok_or(ReadOnlyClusterError::InvalidInput(
                    "No cluster specified, and no default cluster set.".to_owned(),
                ))?
                .clone(),
        };

        let cluster = ctx
            .clusters
            .cluster_config(&name)
            .ok_or(Report::new(ReadOnlyClusterError::NotExists(name)))?;

        let display = global_args
            .out
            .output_string(cluster)
            .change_context(ReadOnlyClusterError::Output)?;

        println!("{}", display);

        Ok(())
    }
}
