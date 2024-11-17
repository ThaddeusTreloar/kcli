use clap::Args;
use error_stack::ResultExt;
use log::trace;
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{BaseConsumer, Consumer},
    ClientConfig, Message,
};
use uuid::Uuid;

use crate::{
    config::{
        clusters::NamedCluster, profiles::reset::ResetStrategy, topics::TopicConfig, ConfigFile,
        Context,
    },
    error::cli::consume::ConsumerError,
    io::serde::Serde,
};

use super::{GlobalArgs, Invoke};

const _REUSE_EXISTING_TOPIC_CONFIG: &str = "Found existing topic config, do you want to reuse?";

#[derive(Args, Debug)]
pub(super) struct ConsumerCommand {
    #[arg(index = 1, help = "Topic to consume from.")]
    topic: String,
    #[arg(short, long, help = "Profile to use.")]
    profile: Option<String>,
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

    fn invoke(
        self,
        ctx: &mut Context,
        global_args: &GlobalArgs,
    ) -> error_stack::Result<(), ConsumerError> {
        let Self {
            topic,
            profile,
            cluster,
            mut reset,
            mut group,
            key_serde,
            value_serde,
        } = self;

        let profile = profile
            .or_else(|| {
                ctx.topics
                    .topic(&topic)
                    .and_then(|p| p.default_profile().cloned())
            })
            .and_then(|p| ctx.profiles.profile(&p));

        let reset_strategy = reset
            .or_else(|| profile.map(|p| p.reset))
            .unwrap_or_default();

        let group_id = group
            .or_else(|| profile.map(|p| p.group_id()))
            .unwrap_or(Uuid::new_v4().to_string());

        let topic_config = match ctx.topics.topic_mut(&topic) {
            Some(topic) => {
                trace!("Using existing topic.");

                if let Some(key_serde) = key_serde {
                    topic.key_serde = key_serde;
                }

                if let Some(value_serde) = value_serde {
                    topic.value_serde = value_serde;
                }

                topic.clone()
            }
            None => {
                let mut config = TopicConfig::default();

                config.key_serde = key_serde.unwrap_or_default();
                config.value_serde = value_serde.unwrap_or_default();

                ctx.topics.add_topic(&topic, config.clone());

                config
            }
        };

        ctx.topics
            .write_out()
            .change_context(ConsumerError::WriteConfig("topics"))?;

        let cluster = if let Some(cluster_name) = cluster {
            ctx.clusters
                .cluster_config(&cluster_name)
                .ok_or(ConsumerError::ClusterNotExists(cluster_name))?
        } else {
            let NamedCluster(_, cluster) = ctx
                .clusters
                .cluster_config_default_or_select()
                .change_context(ConsumerError::FetchDefaultOrSelect)?;

            cluster
        };

        let consumer = ClientConfig::new()
            .set("group.id", group_id)
            .set("bootstrap.servers", cluster.bootstrap_servers.join(","))
            .set("auto.offset.reset", reset_strategy.to_string())
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
                            .key_serde
                            .deserialise_into_string(bytes.to_owned())
                            .change_context(ConsumerError::KeyDeserialisationFailure)?,
                        None => "None".to_owned(),
                    };

                    let value_display = match message.payload() {
                        Some(bytes) => topic_config
                            .value_serde
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
