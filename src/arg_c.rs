use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
	/// Get the key value for a given service_account and account_name.
	/// IMPORTANT: make sure service_account is BEFORE account_name below
	Get {
		/// The service account, e.g., "my_app", "github.com".
		service_account: String,
		/// The account name for this service, e.g., "user_01".
		account_name: String,
	},
}
