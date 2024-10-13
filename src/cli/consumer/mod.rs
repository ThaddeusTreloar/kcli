use clap::Args;
use error_stack::ResultExt;
use log::trace;
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{BaseConsumer, Consumer},
    ClientConfig, Message,
};

use crate::{
    config::{
        clusters::NamedCluster,
        topics::{reset::ResetStrategy, TopicConfig},
        ConfigFile, Context,
    },
    error::cli::consume::ConsumerError,
    io::serde::Serde,
};

use super::Invoke;

const _REUSE_EXISTING_TOPIC_CONFIG: &str = "Found existing topic config, do you want to reuse?";

#[derive(Args, Debug)]
pub(super) struct ConsumerCommand {
    #[arg(index = 1, help = "Topic to consume from.")]
    topic: String,
    #[arg(short, long, help = "Target cluster to consumer from.")]
    cluster: Option<String>,
    #[arg(short, long, help = "Reset strategy to use when consuming.")]
    reset: Option<ResetStrategy>,
    #[arg(short, long, help = "Consumer group to use.")]
    group: Option<String>,
    #[arg(short, long, help = "Key deserialiser.")]
    key_serde: Option<Serde>,
    #[arg(short, long, help = "Value deserialiser.")]
    value_serde: Option<Serde>,
}

impl Invoke for ConsumerCommand {
    type E = ConsumerError;

    fn invoke(self, ctx: &mut Context) -> error_stack::Result<(), ConsumerError> {
        let Self {
            topic,
            cluster,
            reset,
            group,
            key_serde,
            value_serde,
        } = self;

        let topic_config = match ctx.topics_mut().topic_mut(&topic) {
            Some(topic) => {
                trace!("Using existing topic.");

                topic.set_group(group.into());

                if let Some(reset) = reset {
                    topic.set_reset(reset);
                }

                if let Some(key_serde) = key_serde {
                    topic.set_key_serde(key_serde);
                }

                if let Some(value_serde) = value_serde {
                    topic.set_value_serde(value_serde);
                }

                topic.clone()
            }
            None => {
                let mut config = TopicConfig::default();

                config.set_group(group.into());
                config.set_reset(reset.unwrap_or_default());
                config.set_key_serde(key_serde.unwrap_or_default());
                config.set_value_serde(value_serde.unwrap_or_default());

                ctx.topics_mut().add_topic(&topic, config.clone());

                config
            }
        };

        ctx.topics()
            .write_out()
            .change_context(ConsumerError::WriteConfig("topics"))?;

        let cluster = if let Some(cluster_name) = cluster {
            ctx.clusters()
                .cluster_config(&cluster_name)
                .ok_or(ConsumerError::ClusterNotExists(cluster_name))?
        } else {
            let NamedCluster(_, cluster) = ctx
                .clusters()
                .cluster_config_default_or_select()
                .change_context(ConsumerError::FetchDefaultOrSelect)?;

            cluster
        };

        let consumer = ClientConfig::new()
            .set("group.id", topic_config.group_id())
            .set("bootstrap.servers", cluster.bootstrap_servers().join(","))
            .set("auto.offset.reset", topic_config.reset_string())
            .set_log_level(RDKafkaLogLevel::Emerg)
            .create::<BaseConsumer>()
            .change_context(ConsumerError::CreateConsumer)?;

        let topics = vec![topic.as_str()];

        consumer
            .subscribe(&topics)
            .change_context(ConsumerError::ConsumerFailure)?;

        for message_result in consumer.iter() {
            match message_result {
                Err(e) => Err(e).change_context(ConsumerError::ConsumerFailure)?,
                Ok(message) => {
                    let key_display = match message.key() {
                        Some(bytes) => topic_config
                            .key_serde()
                            .deserialise_into_string(bytes.to_owned())
                            .change_context(ConsumerError::KeyDeserialisationFailure)?,
                        None => "None".to_owned(),
                    };

                    let value_display = match message.payload() {
                        Some(bytes) => topic_config
                            .value_serde()
                            .deserialise_into_string(bytes.to_owned())
                            .change_context(ConsumerError::KeyDeserialisationFailure)?,
                        None => "None".to_owned(),
                    };

                    println!("{}: {}", key_display, value_display);
                }
            }
        }

        Ok(())
    }
}
