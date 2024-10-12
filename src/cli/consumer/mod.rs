use clap::Args;
use error_stack::{Report, ResultExt};
use log::{warn, error};
use rdkafka::{config::RDKafkaLogLevel, consumer::{BaseConsumer, Consumer}, ClientConfig, Message};
use util::ResetStrategy;
use uuid::Uuid;

mod util;

use crate::{cli::util::get_user_choice, config::Context, error::cli::consume::ConsumerError, io::output::Output};

use super::Invoke;

const SELECT_CLUSTER_PROMPT: &str = "Select cluster";

#[derive(Args, Debug)]
pub(super) struct ConsumerCommand {
    #[arg(index = 1, help = "Topic to consume from.")]
    topic: String,
    #[arg(short, long, help = "Target cluster to consumer from.")]
    cluster: Option<String>,
    #[arg(short, long, default_value_t, help = "Reset strategy to use when consuming.")]
    reset: ResetStrategy,
    #[arg(short, long, help = "Consumer group to use.")]
    group: Option<String>,
}

impl Invoke for ConsumerCommand {
    type E = ConsumerError;

    fn invoke(self, mut ctx: Context) -> error_stack::Result<(), ConsumerError> {
        let Self { topic, cluster, reset, group } = self;

        let cluster = match cluster {
            Some(cluster_name) => ctx.clusters().cluster_config(&cluster_name)
                .ok_or(Report::new(ConsumerError::ClusterNotExists(cluster_name)))?,
            None => if ctx.clusters().default().is_some() {
                ctx.clusters().get_default().expect("Unexpected error while fetching default cluster.")
            } else {
                warn!("No default cluster set, and no cluster provided.");

                let choices = ctx.clusters().list_clusters();

                if choices.is_empty() {
                    Err(ConsumerError::ClusterNotExists("No cluster provided, and no clusters available.".to_owned()))?
                }

                let choice = get_user_choice(SELECT_CLUSTER_PROMPT, choices)
                    .change_context(ConsumerError::InputError("cluster choice"))?;

                ctx.clusters().cluster_config(choice).expect("Unexpected error.")
            }
        };

        let group_id = group.unwrap_or(Uuid::new_v4().to_string());

        let consumer = ClientConfig::new()
            .set("group.id", group_id)
            .set("bootstrap.servers", cluster.bootstrap_servers().join(","))
            .set("auto.offset.reset", reset.to_string())
            .set_log_level(RDKafkaLogLevel::Emerg)
            .create::<BaseConsumer>()
            .change_context(ConsumerError::CreateConsumer)?;
        
        let topics = vec![topic.as_str()];

        consumer.subscribe(&topics)
            .change_context(ConsumerError::ConsumerFailure)?;

        for message_result in consumer.iter() {
            match message_result {
                Err(e) => Err(e).change_context(ConsumerError::ConsumerFailure)?,
                Ok(message) => {
                    let key = match message.key_view::<str>() {
                        Some(Ok(key)) => Some(key),
                        None => None,
                        Some(Err(e)) => Err(e).change_context(ConsumerError::KeyDeserialisationFailure)?,
                    };

                    let value = match message.payload_view::<str>() {
                        Some(Ok(value)) => Some(value),
                        None => None,
                        Some(Err(e)) => Err(e).change_context(ConsumerError::KeyDeserialisationFailure)?,
                    };

                    println!("{:?}: {:?}", key, value);
                }
            }
        }

        Ok(())
    }
}