use acl::AclCommand;
use config::ConfigCommand;
use consumer::ConsumerCommand;
use group::GroupCommand;
use inquire::{error::InquireError, Select};
use clap::{Parser, Subcommand};
use producer::ProducerCommand;
use topic::TopicCommand;

use crate::error::cli::ExecutionError;

mod acl;
mod config;
mod consumer;
mod group;
mod producer;
mod topic;
mod util;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub (crate) struct Cli {
    #[command(subcommand)]
    command: RootCommand,
}

#[derive(Subcommand, Debug)]
enum RootCommand {
    Acl(AclCommand),
    Config(ConfigCommand),
    Consumer(ConsumerCommand),
    Group(GroupCommand),
    Producer(ProducerCommand),
    Topic(TopicCommand),
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

impl Cli {
    pub (crate) fn execute(self) -> error_stack::Result<(), ExecutionError> {
        match self.command {
            RootCommand::Acl(command) => command.execute(),
            RootCommand::Config(command) => command.execute(),
            RootCommand::Consumer(command) => command.execute(),
            RootCommand::Group(command) => command.execute(),
            RootCommand::Producer(command) => command.execute(),
            RootCommand::Topic(command) => command.execute(),
        }
    }
}