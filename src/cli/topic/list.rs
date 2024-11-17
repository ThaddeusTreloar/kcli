use std::time::Duration;

use clap::Args;
use error_stack::ResultExt;
use rdkafka::{
    admin::AdminClient, client::DefaultClientContext, config::RDKafkaLogLevel, ClientConfig,
};
use regex::Regex;
use tabled::{
    builder::Builder,
    settings::{Panel, Style},
};

use crate::{
    cli::{topic::INTERNAL_TOPIC_REGEX, GlobalArgs, Invoke},
    config::{clusters::NamedCluster, Context},
    error::cli::config::topic::ReadOnlyTopicError,
};

#[derive(Debug, Args)]
pub(super) struct ListTopics {
    #[arg(short, long, help = "Target cluster to consumer from.")]
    cluster: Option<String>,
    #[arg(long, help = "Whether to exclude internal topics.")]
    exclude_internal: bool,
    #[arg(short, long, help = "Exclude topics with prefix")]
    exclude_prefix: Option<String>,
    #[arg(short, long, help = "Include topics with prefix.")]
    include_prefix: Option<String>,
    #[arg(short, long, help = "Regex to match topics with.")]
    regex: Option<String>,
}

impl Invoke for ListTopics {
    type E = ReadOnlyTopicError;

    fn invoke(
        self,
        ctx: &mut Context,
        global_args: &GlobalArgs,
    ) -> error_stack::Result<(), ReadOnlyTopicError> {
        let Self {
            mut cluster,
            exclude_prefix,
            include_prefix,
            regex,
            exclude_internal,
        } = self;

        let cluster_config = if let Some(cluster_name) = &cluster {
            ctx.clusters.cluster_config(cluster_name).ok_or(
                ReadOnlyTopicError::ClusterNotExists(cluster_name.to_owned()),
            )?
        } else {
            let NamedCluster(name, cluster_config) = ctx
                .clusters
                .cluster_config_default_or_select()
                .change_context(ReadOnlyTopicError::FetchDefaultOrSelect)?;

            cluster.replace(name);

            cluster_config
        };

        let admin_client = ClientConfig::new()
            .set(
                "bootstrap.servers",
                cluster_config.bootstrap_servers.join(","),
            )
            .set_log_level(RDKafkaLogLevel::Emerg)
            .create::<AdminClient<DefaultClientContext>>()
            .change_context(ReadOnlyTopicError::AdminClient)?;

        let metadata = admin_client
            .inner()
            .fetch_metadata(None, Duration::from_millis(2500))
            .change_context(ReadOnlyTopicError::AdminClient)?;

        let internal_topic_regex =
            Regex::new(INTERNAL_TOPIC_REGEX).expect("Failed to compile inbuilt regex");

        let is_internal_topic = |t: &&str| internal_topic_regex.is_match(t);

        let exclude = exclude_prefix.is_some();
        let exclude_prefix = exclude_prefix.unwrap_or("".to_owned());

        let include = include_prefix.is_some();
        let include_prefix = include_prefix.unwrap_or("".to_owned());

        let user_regex = match regex {
            None => None,
            Some(s) => Some(Regex::new(&s).change_context(ReadOnlyTopicError::CompileRegex(s))?),
        };

        let mut table_builder = Builder::default();

        metadata
            .topics()
            .iter()
            .map(|t| t.name())
            .filter(|t| t.starts_with(&include_prefix) || !include)
            .filter(|t| !t.starts_with(&exclude_prefix) || !exclude)
            .filter(|t| !is_internal_topic(t) || !exclude_internal)
            .filter(|t| match &user_regex {
                None => true,
                Some(re) => re.is_match(t),
            })
            .for_each(|t| table_builder.push_record(vec![t]));

        let mut table = table_builder.build();

        let count = table.count_rows();
        let width = table.total_width();
        let seperator = (0..width).map(|_| "-").collect::<String>();

        table
            .with(Style::rounded())
            .with(Panel::horizontal(count, seperator))
            .with(Panel::header("topic"))
            .with(Panel::footer(format!(
                "Count: {}, Cluster: {}",
                count,
                cluster.as_ref().unwrap()
            )))
            .get_config();
        table.count_rows();

        println!("{}", table);

        Ok(())
    }
}
