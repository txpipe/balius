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
        #[arg(short, long, env = "WASM_CONFIG_PATH")]
        config: Option<String>,
        
        /// Port to use for the JSON-RPC server
        #[arg(short, long, default_value = "3000", env = "PORT")]
        port: u16,

        /// UTXoRCP endpoint URL
        #[arg(long, env = "UTXO_URL")]
        utxo_url: String,

        /// UTXoRCP API key
        #[arg(long, env = "UTXO_API_KEY")]
        utxo_api_key: String,
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
            utxo_url,
            utxo_api_key,
        } => command::test::execute(
            config,
            port,
            utxo_url,
            utxo_api_key
        ).await,
    }
}
