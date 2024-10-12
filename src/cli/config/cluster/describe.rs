use clap::Args;
use error_stack::{Report, ResultExt};

use crate::{
    cli::Invoke, config::Context, error::cli::config::cluster::ReadOnlyClusterError,
    io::output::Output,
};

#[derive(Debug, Args)]
pub(super) struct DescribeCluster {
    #[arg(index = 1, help = "Logical name for the cluster.")]
    name: Option<String>,
    #[arg(
        short,
        long,
        global = true,
        default_value_t,
        help = "Output format for commands."
    )]
    out: Output,
}

impl Invoke for DescribeCluster {
    type E = ReadOnlyClusterError;

    fn invoke(self, ctx: Context) -> error_stack::Result<(), ReadOnlyClusterError> {
        let Self { name, out } = self;

        let name = match name {
            Some(name) => name,
            None => ctx
                .clusters()
                .default()
                .ok_or(ReadOnlyClusterError::InvalidInput(
                    "No cluster specified, and no default cluster set.".to_owned(),
                ))?
                .clone(),
        };

        let cluster = ctx
            .clusters()
            .cluster_config(&name)
            .ok_or(Report::new(ReadOnlyClusterError::NotExists(name)))?;

        let display = out
            .write_as_string(cluster)
            .change_context(ReadOnlyClusterError::Output)?;

        println!("{}", display);

        Ok(())
    }
}
