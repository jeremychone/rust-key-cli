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
		Commands::Get => {
			println!("'get' command not implemented yet.");
		}
	}

	Ok(())
}
