use clap::{Parser, Subcommand};

mod command;

#[derive(Debug, Subcommand)]
enum BaliusCommand {
    Build,
    Init,
    Test, // Added Test command
}

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    command: BaliusCommand,
}

fn main() {
    let args = Args::parse_from(std::env::args().skip(1));

    match args.command {
        BaliusCommand::Build => command::build::execute(),
        BaliusCommand::Init => command::init::execute(),
        BaliusCommand::Test => command::test::execute(),
    }
}
