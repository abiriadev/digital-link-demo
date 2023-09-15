use check_digit_calc::Gtin;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
struct Arg {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	Validate { gtin: String },
	Calc { partial_gtin: String },
}

fn main() {
	let arg = Arg::parse();

	match arg.command {
		Commands::Validate { gtin } => match Gtin::try_from(gtin.as_str()) {
			Ok(_) => println!("OK"),
			Err(e) => println!("validation failed. cause: {e}"),
		},
		Commands::Calc { partial_gtin } =>
			match Gtin::calc_check_digit_from_str(&partial_gtin) {
				Ok(d) => println!("check digit: {d}"),
				Err(e) => println!("validation failed. cause: {e}"),
			},
	}
}
