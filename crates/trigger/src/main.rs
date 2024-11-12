
use anyhow::Error;
use clap::Parser;
use spin_trigger::cli::FactorsTriggerCommand;
use trigger::{CustomFactorsBuilder, LineTrigger};

pub type Command = FactorsTriggerCommand<LineTrigger, CustomFactorsBuilder>;   

#[tokio::main]
async fn main() -> Result<(), Error> {
    let t = Command::parse();
    t.run().await
}