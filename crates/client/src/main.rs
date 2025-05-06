use clap::Parser;
use client::Cli;
use common::schemas::{Agent, Response};
use uuid::Uuid;

fn main() {
    //client::create_client_identity();

    let args = Cli::parse();

    let api_client = client::init();

    match args.command {
        client::Commands::Agents(agents) => {
            let cmd = agents.command;
            match cmd {
                client::AgentCommands::Fetch { uuid } => match Uuid::parse_str(&uuid) {
                    Err(err) => {
                        panic!("Error: couldn't parse uuid: {:?}", err);
                    }
                    Ok(uuid) => {
                        todo!()
                    }
                },

                client::AgentCommands::ps => {}
            }
        }
    }
}
