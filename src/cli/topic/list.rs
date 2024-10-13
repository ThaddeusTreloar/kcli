use std::time::Duration;

use clap::Args;
use error_stack::ResultExt;
use rdkafka::{
    admin::AdminClient, client::DefaultClientContext, config::RDKafkaLogLevel, ClientConfig,
};

use crate::{cli::Invoke, config::Context, error::cli::config::topic::ReadOnlyTopicError};

#[derive(Debug, Args)]
pub(super) struct ListTopics {
    #[arg(short, long, help = "Target cluster to consumer from.")]
    cluster: Option<String>,
    #[arg(short, long, help = "Whether to exclude internal topics.")]
    exclude_internal: bool,
}

impl Invoke for ListTopics {
    type E = ReadOnlyTopicError;

    fn invoke(self, mut ctx: &mut Context) -> error_stack::Result<(), ReadOnlyTopicError> {
        let Self {
            cluster,
            exclude_internal,
        } = self;

        let cluster = if let Some(cluster_name) = cluster {
            ctx.clusters()
                .cluster_config(&cluster_name)
                .ok_or(ReadOnlyTopicError::ClusterNotExists(cluster_name))?
        } else {
            ctx.clusters()
                .cluster_config_default_or_select()
                .change_context(ReadOnlyTopicError::FetchDefaultOrSelect)?
        };

        let admin_client = ClientConfig::new()
            .set("bootstrap.servers", cluster.bootstrap_servers().join(","))
            .set_log_level(RDKafkaLogLevel::Emerg)
            .create::<AdminClient<DefaultClientContext>>()
            .change_context(ReadOnlyTopicError::AdminClient)?;

        let metadata = admin_client
            .inner()
            .fetch_metadata(None, Duration::from_millis(2500))
            .change_context(ReadOnlyTopicError::AdminClient)?;

        let exclude = exclude_internal;
        let exclude_prefix = "_";

        metadata
            .topics()
            .iter()
            .map(|t| t.name())
            .filter(|t| !t.starts_with(exclude_prefix) || !exclude)
            .for_each(|topic| println!("{}", topic));

        Ok(())
    }
}
