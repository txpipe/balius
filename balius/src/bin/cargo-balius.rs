use clap::Parser;

mod command;

#[derive(Debug, Parser)]
enum BaliusCommand {
    /// Build the Balius project
    Build,
    /// Initialize a new Balius project
    #[command(trailing_var_arg = true)]
    Init {
        /// Project name
        #[arg(allow_hyphen_values = false)]
        project_name: Vec<String>,
    },
    /// Run the Balius test server
    #[command(arg_required_else_help = true)]
    Test {
        /// Path to a custom configuration file
        #[arg(short, long)]
        config: Option<String>,
        
        /// Port to use for the JSON-RPC server
        #[arg(short, long, default_value = "3000", env = "PORT")]
        port: u16,

        /// Ledger endpoint URL
        #[arg(long, default_value = "https://preview.utxorpc-v0.demeter.run", env = "LEDGER_URL")]
        ledger_url: String,

        /// Ledger API key
        #[arg(long, env = "LEDGER_API_KEY")]
        ledger_api_key: String,

        /// Chainsync endpoint URL
        #[arg(long, default_value = "https://preview.utxorpc-v0.demeter.run", env = "CHAINSYNC_URL")]
        chainsync_url: String,

        /// Chainsync API key
        #[arg(long, env = "CHAINSYNC_API_KEY")]
        chainsync_api_key: String,
    },
}

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    command: BaliusCommand,
}

#[tokio::main]
async fn main() {
    let args = Args::parse_from(std::env::args().skip(1));

    match args.command {
        BaliusCommand::Build => command::build::execute(),
        BaliusCommand::Init { project_name } => command::init::execute(project_name),
        BaliusCommand::Test {
            config,
            port,
            ledger_url,
            ledger_api_key,
            chainsync_url,
            chainsync_api_key,
        } => command::test::execute(
            config,
            port,
            ledger_url,
            ledger_api_key,
            chainsync_url,
            chainsync_api_key
        ).await,
    }
}
