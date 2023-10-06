use clap::{Args, Parser, Subcommand};
use env_logger::Builder as LoggerBuilder;
use log::LevelFilter;

mod commands;
mod host;
mod mail;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    global: GlobalArgs,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
struct GlobalArgs {
    /// Set the log level. Defaults to INFO. Use Multiple times to increase the log output.
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List email addresses and names in the dataset and export a CSV file.
    List(ListArgs),
    /// Extract a Cosmograph network graph.
    Cosmograph(CosmographArgs),
    /// Extract all attachments from emails.
    Attachment(AttachmentArgs),
}

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
struct ListArgs {
    /// Name of CSV output file.
    #[arg(short, long, value_name = "FILE", default_value = "out.csv")]
    output: String,

    /// Path to directory containing EML  files.
    input: String,
}

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
struct CosmographArgs {
    /// Name of CSV output file.
    #[arg(short, long, value_name = "FILE", default_value = "out.csv")]
    output: String,

    /// Path to directory containing EML  files.
    input: String,
}

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
struct AttachmentArgs {
    /// Target path where to extract attachments to.
    #[arg(short, long, value_name = "TARGET", default_value = "out")]
    outdir: String,

    /// Path to directory containing EML  files.
    input: String,
}

fn main() {
    let cli = Cli::parse();

    let mut builder = LoggerBuilder::new();
    match cli.global.verbose {
        0 => builder.filter_level(LevelFilter::Info),
        _ => builder.filter_level(LevelFilter::Debug),
    };

    builder.init();

    match &cli.command {
        Commands::List(args) => {
            commands::list(&args.output, &args.input).unwrap();
        }

        Commands::Cosmograph(args) => {
            commands::cosmograph(&args.output, &args.input).unwrap();
        }

        Commands::Attachment(args) => {
            commands::attachment(&args.outdir, &args.input).unwrap();
        }
    }
}
