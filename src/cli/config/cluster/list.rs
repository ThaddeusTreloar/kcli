use clap::Args;
use error_stack::ResultExt;

use crate::{
    cli::Invoke, config::Context, error::cli::config::cluster::ReadOnlyClusterError,
    io::output::Output,
};

#[derive(Debug, Args)]
pub(super) struct ListCluster {
    #[arg(
        short,
        long,
        global = true,
        default_value_t,
        help = "Output format for commands."
    )]
    out: Output,
}

impl Invoke for ListCluster {
    type E = ReadOnlyClusterError;

    fn invoke(self, ctx: &mut Context) -> error_stack::Result<(), ReadOnlyClusterError> {
        let Self { out } = self;

        let clusters = ctx.clusters().list_clusters();

        let display = out
            .write_as_string(&clusters)
            .change_context(ReadOnlyClusterError::Output)?;

        println!("{}", display);

        Ok(())
    }
}
