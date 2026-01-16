use clap::Parser;
use std::{fs::File, io::{BufReader, BufWriter}};

fn main() -> Result<(), seapeaewe::AssembleError> {
    // Parse command line arguments and create asm configuration
    let cli = CliArgs::parse();
    let config = parse_config(&cli);

    // Configure the input and output streams
    let input = BufReader::new(File::open(cli.input_file)?);
    let output = BufWriter::new(File::create(cli.output_file)?);

    // Dispatch request to the assembler library
    seapeaewe::assemble(input, output, &config)?;

    Ok(())
}

/// Parses command line arguments into a configuration for the assembler.
fn parse_config(cli: &CliArgs) -> seapeaewe::Config {
    seapeaewe::Config {
        verbose: cli.verbose,
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    input_file: String,

    #[arg(short, default_value = "a.bin")]
    output_file: String,

    #[arg(short)]
    verbose: bool,
}
