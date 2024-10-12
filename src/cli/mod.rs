use acl::AclCommand;
use clap::{Parser, Subcommand};
use completions::CompletionsCommand;
use config::ConfigCommand;
use consumer::ConsumerCommand;
use error_stack::ResultExt;
use group::GroupCommand;
use producer::ProducerCommand;
use topic::TopicCommand;

use crate::{config::Context, error::cli::ExecutionError, io::output::Output};

mod acl;
mod completions;
mod config;
mod consumer;
mod group;
mod producer;
mod topic;
pub mod util;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, author)]
pub struct Cli {
    #[command(subcommand)]
    command: RootCommand,
    #[arg(long, global = true, help = "Set log level to 'INFO'.")]
    verbose: bool,
    #[arg(
        long,
        global = true,
        default_value_t,
        help = "Output format for commands."
    )]
    out: Output,
}

#[derive(Subcommand, Debug)]
enum RootCommand {
    #[command(about = "Manage Kafka ACLS")]
    Acl(AclCommand),
    #[command(about = "Manage kcli configurations")]
    Config(ConfigCommand),
    #[command(about = "Consumer messages from a topic")]
    Consume(ConsumerCommand),
    #[command(about = "Manage Kafka consumer group")]
    Group(GroupCommand),
    #[command(about = "Produce messages to a topic")]
    Produce(ProducerCommand),
    #[command(about = "Manage Kafka topics")]
    Topic(TopicCommand),
    #[command(about = "Print out shell completions")]
    Completions(CompletionsCommand),
}
/*
kafka-broker-api-versions.sh
kafka-cluster.sh
kafka-configs.sh
kafka-consumer-perf-test.sh
kafka-delegation-tokens.sh
kafka-delete-records.sh
kafka-dump-log.sh
kafka-e2e-latency.sh
kafka-features.sh
kafka-get-offsets.sh
kafka-jmx.sh
kafka-leader-election.sh
kafka-log-dirs.sh
kafka-metadata-quorum.sh
kafka-metadata-shell.sh
kafka-mirror-maker.sh
kafka-producer-perf-test.sh
kafka-reassign-partitions.sh
kafka-replica-verification.sh
kafka-run-class.sh
kafka-storage.sh
kafka-streams-application-reset.sh
kafka-transactions.sh
kafka-verifiable-consumer.sh
kafka-verifiable-producer.sh
*/

pub trait Invoke {
    type E: std::error::Error;

    fn invoke(self, ctx: Context) -> error_stack::Result<(), Self::E>;
}

impl Invoke for Cli {
    type E = ExecutionError;

    fn invoke(self, ctx: Context) -> error_stack::Result<(), ExecutionError> {
        match self.command {
            RootCommand::Acl(command) => command.execute(),
            RootCommand::Config(command) => command.invoke(ctx),
            RootCommand::Consume(command) => command
                .invoke(ctx)
                .change_context(ExecutionError::ExecutionFailed("consume")),
            RootCommand::Group(command) => command.execute(),
            RootCommand::Produce(command) => command.execute(),
            RootCommand::Topic(command) => command.execute(),
            RootCommand::Completions(command) => command.execute(),
        }
    }
}
