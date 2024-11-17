use clap::Args;
use error_stack::ResultExt;

use crate::{
    cli::{GlobalArgs, Invoke},
    config::Context,
    error::cli::config::cluster::ReadOnlyClusterError,
    io::output::Output,
};

#[derive(Debug, Args)]
pub(super) struct ListCluster {}

impl Invoke for ListCluster {
    type E = ReadOnlyClusterError;

    fn invoke(
        self,
        ctx: &mut Context,
        global_args: &GlobalArgs,
    ) -> error_stack::Result<(), ReadOnlyClusterError> {
        let Self {} = self;

        let clusters = ctx.clusters.list_clusters();

        let display = global_args
            .out
            .output_string(&clusters)
            .change_context(ReadOnlyClusterError::Output)?;

        println!("{}", display);

        Ok(())
    }
}
