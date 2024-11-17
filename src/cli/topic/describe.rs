use std::{collections::HashMap, time::Duration};

use clap::Args;
use error_stack::{Report, ResultExt};
use futures::executor;
use rdkafka::{
    admin::{AdminClient, AdminOptions, OwnedResourceSpecifier, ResourceSpecifier}, client::DefaultClientContext, config::RDKafkaLogLevel, ClientConfig,
};
use regex::Regex;
use serde::Serialize;
use tabled::{
    grid::records::ExactRecords, settings::{Panel, Style}, Table, Tabled
};

use crate::{
    cli::{topic::INTERNAL_TOPIC_REGEX, GlobalArgs, Invoke},
    config::{clusters::NamedCluster, Context},
    error::cli::config::topic::ReadOnlyTopicError,
};

#[derive(Debug, Serialize)]
struct PartitionRow {
    id: i32,
    leader: i32,
    replicas: String,
    isr: String
}

#[derive(Debug, Tabled)]
struct TopicRow {
    topic: String,
    partition_count: usize,
    replication_factor: usize,
    configs: String,
    partitions: String,
}

#[derive(Debug, Args)]
pub(super) struct DescribeTopic {
    #[arg(
        index = 1, 
        conflicts_with_all=["exclude_internal","exclude_prefix","include_prefix","regex",], 
        help = "Logical name for the cluster."
    )]
    name: Option<String>,
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

impl Invoke for DescribeTopic {
    type E = ReadOnlyTopicError;

    fn invoke(self, ctx: &mut Context, global_args: &GlobalArgs) -> error_stack::Result<(), ReadOnlyTopicError> {
        let Self {
            name,
            mut cluster,
            exclude_prefix,
            include_prefix,
            regex,
            exclude_internal,
        } = self;

        let filters = format!(
            "specific: '{}', exclude: '{}', include: '{}', regex: '{}'", 
            name.as_ref().unwrap_or(&"None".to_owned()), 
            exclude_prefix.as_ref().unwrap_or(&"None".to_owned()), 
            include_prefix.as_ref().unwrap_or(&"None".to_owned()), 
            regex.as_ref().unwrap_or(&"None".to_owned())
        );

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

        let is_internal_topic = |t: &str| internal_topic_regex.is_match(t);

        let exclude = exclude_prefix.is_some();
        let exclude_prefix_parse = exclude_prefix.unwrap_or("".to_owned());

        let include = include_prefix.is_some();
        let include_prefix_parsed = include_prefix.unwrap_or("".to_owned());

        let user_regex = match regex {
            None => None,
            Some(s) => Some(Regex::new(&s).change_context(ReadOnlyTopicError::CompileRegex(s))?),
        };

        let topics = metadata
            .topics()
            .iter()
            .map(|t|
                (t.name(), t.partitions()))
            .filter(|t| t.0.starts_with(&include_prefix_parsed) || !include)
            .filter(|t| !t.0.starts_with(&exclude_prefix_parse) || !exclude)
            .filter(|t| !is_internal_topic(t.0) || !exclude_internal)
            .filter(|t| match &user_regex {
                None => true,
                Some(re) => re.is_match(t.0),
            })
            .filter(|t| match &name {
                None => true,
                Some(name) => t.0 == name
            })
            .collect::<HashMap<_, _>>();

        if topics.is_empty() {
            Err(Report::new(ReadOnlyTopicError::NotExists(filters)))?
        }
        
        let resources = topics
            .keys()
            .map(|t|t.to_owned())
            .map(ResourceSpecifier::Topic)
            .collect::<Vec<_>>();

        let admin_options = AdminOptions::new();

        let configs = executor::block_on(admin_client.describe_configs(resources.iter(), &admin_options))
            .change_context(ReadOnlyTopicError::AdminClient)?
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .change_context(ReadOnlyTopicError::AdminClient)?;

        let mut cleaned_config = configs.into_iter()
            .map(|c|(
                match &c.specifier {
                    OwnedResourceSpecifier::Topic(name) => name.to_owned(),
                    _ => panic!("") 
                }, 
                c.entry_map().into_iter()
                .map(|(k, v)| (k.to_owned(), v.value.clone().unwrap_or("None".to_owned())))
                .collect::<HashMap<_, _>>(),
            ))
            .collect::<HashMap<_, _>>();
        
        let joined_data = topics
            .into_iter()
            .map(|(k, v)|(k, (v, cleaned_config.remove(k).unwrap())))
            .map(|(k, (p, c))| TopicRow {
                topic: k.to_owned(),
                partition_count: p.count_rows(),
                replication_factor: p[0].replicas().count_rows(),
                //leader: String,
                configs: serde_json::to_string_pretty(&c).expect("Failed ser hashmap"),
                partitions: serde_json::to_string_pretty(&p.iter()
                    .map(|p|(
                        PartitionRow {
                            id: p.id(),
                            leader: p.leader(),
                            replicas: format!("[{}]", p.replicas().iter().map(|x|x.to_string()).collect::<Vec<_>>().join(",")),
                            isr: format!("[{}]", p.isr().iter().map(|x|x.to_string()).collect::<Vec<_>>().join(",")),
                        }
                    )).collect::<Vec<_>>()).expect("Failed serialise hashmap")
            })
            .collect::<Vec<_>>();

        println!("{}", joined_data.count_rows());
        
        let mut table = Table::new(joined_data.iter());

        table
            .with(Style::modern_rounded())
            .with(Panel::footer(format!(
                "Count: {}, Cluster: {}",
                joined_data.count_rows(),
                cluster.as_ref().unwrap()
            )))
            .get_config();
        table.count_rows();

        println!("{}", table);

        Ok(())
    }
}
