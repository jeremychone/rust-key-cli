// region:    --- Modules

mod arg_c;
mod error;

pub use self::error::{Error, Result};

use crate::arg_c::{Cli, Commands};
use clap::Parser;

// endregion: --- Modules

fn main() {
	// -- Cli
	let cli = Cli::parse();

	// -- Command
	if let Err(err) = exec_cli(cli) {
		println!("key-cli error - {err}");
		std::process::exit(1);
	}
}

fn exec_cli(cli: Cli) -> Result<()> {
	match cli.command {
		Commands::Get {
			service_account,
			account_name,
		} => {
			let entry = keyring::Entry::new(&service_account, &account_name)?;
			let pwd = entry
				.get_password()
				.map_err(|_| format!("No matching entry found for {service_account}/{account_name}"))?;
			println!("{pwd}");
		}
	}
	Ok(())
}
