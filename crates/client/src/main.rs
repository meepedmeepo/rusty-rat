use clap::Parser;
use client::Cli;

fn main() {
    //client::create_client_identity();

    let args = Cli::parse();

    match args.command {
        client::Commands::Agents(agents) => {
            let cmd = agents.command;
            match cmd {
                client::AgentCommands::Fetch { uuid } => {}

                client::AgentCommands::ps => {}
            }
        }
    }
}
