use clap::{Parser, Subcommand};
use kvs::KvStore;
use std::process::exit;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set value in kvs
    Set { key: String, value: String },

    /// Get value from kvs
    Get { key: String },

    /// Remove value from kvs
    Rm { key: String },
}

fn main() {
    let mut kvs = KvStore::new();
    let args = Args::parse();

    match args.command {
        Commands::Set { key, value } => {
            kvs.set(key, value);

            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Get { key } => {
            kvs.get(key);

            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Rm { key } => {
            kvs.remove(key);

            eprintln!("unimplemented");
            exit(1);
        }
    }
}
