use clap::{Command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Commands {
    ///Queries server api to get information on agents
    Agents(AgentsArgs),
}

#[derive(Debug, clap::Args, Clone)]
pub struct JobArgs {
    #[command(subcommand)]
    pub command: JobCommands,
}

#[allow(non_camel_case_types)]
#[derive(clap::Subcommand, Debug, Clone)]
pub enum JobCommands {}

#[derive(Debug, clap::Args, Clone)]
pub struct AgentsArgs {
    #[command(subcommand)]
    pub command: AgentCommands,

    ///path to file to output agent information to
    #[arg(short, long)]
    pub output: Option<String>,
}

#[allow(non_camel_case_types)]
#[derive(clap::Subcommand, Debug, Clone)]
pub enum AgentCommands {
    ///Fetches information on agent with a given UUID
    Fetch {
        uuid: String,
    },
    ps,
}
