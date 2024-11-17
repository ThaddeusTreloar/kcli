use std::process::exit;

use acl::AclCommand;
use clap::{Parser, Subcommand};
use completions::CompletionsCommand;
use config::ConfigCommand;
use consumer::ConsumerCommand;
use error_stack::ResultExt;
use group::GroupCommand;
use producer::ProducerCommand;
use simplelog::LevelFilter;
use topic::TopicCommand;

use crate::{
    config::Context,
    error::{cli::ExecutionError, handle_expect_report},
    io::output::Output,
    util::init_logging,
};

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

    fn invoke(
        self,
        ctx: &mut Context,
        global_args: &GlobalArgs,
    ) -> error_stack::Result<(), Self::E>;
}

impl Cli {
    pub fn invoke_root(self) {
        let Self {
            command,
            verbose,
            out,
        } = self;

        let log_level = if verbose {
            LevelFilter::Info
        } else {
            LevelFilter::Error
        };

        init_logging(log_level);

        let mut ctx = match Context::init() {
            Ok(ctx) => ctx,
            Err(e) => {
                handle_expect_report(&e);
                exit(1);
            }
        };

        let global_args = GlobalArgs { out };

        match command {
            RootCommand::Acl(command) => command.execute(),
            RootCommand::Config(command) => command.invoke(&mut ctx, &global_args),
            RootCommand::Consume(command) => command
                .invoke(&mut ctx, &global_args)
                .change_context(ExecutionError::ExecutionFailed("consume")),
            RootCommand::Group(command) => command.execute(),
            RootCommand::Produce(command) => command.execute(),
            RootCommand::Topic(command) => command.invoke(&mut ctx, &global_args),
            RootCommand::Completions(command) => command.execute(),
        }
        .inspect_err(handle_expect_report);

        ctx.write_out().inspect_err(handle_expect_report);
    }
}

#[derive(Debug, Default)]
pub struct GlobalArgs {
    pub out: Output,
}
