// region:    --- Modules

mod arg_c;
mod error;

pub use self::error::{Error, Result};

use crate::arg_c::{Cli, Commands};
use clap::Parser;

// endregion: --- Modules

fn main() -> Result<()> {
	// -- Cli
	let cli = Cli::parse();

	// -- Command
	match cli.command {
		Commands::Get {
			service_account,
			account_name,
		} => {
			println!("->> {service_account} - {account_name}");

			// let entry = keyring::Entry::new(&service_account, &account_name);
			// match entry.get_password() {
			// 	Ok(password) => {
			// 		println!("{password}");
			// 	}
			// 	Err(keyring::Error::NoEntry) => {
			// 		eprintln!(
			// 			"No secret found for service_account '{service_account}' and account_name '{account_name}'"
			// 		);
			// 	}
			// 	Err(e) => return Err(e.into()),
			// }
		}
	}

	Ok(())
}
