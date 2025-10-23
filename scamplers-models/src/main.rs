use std::fs;

use clap::Parser;
use scamplers_models::*;
use schemars::schema_for;

#[derive(clap::Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long, default_value_t = String::from("scamplers-jsonschema.json"))]
    output: String,
}

fn main() {
    let Cli { output } = Cli::parse();
    // These are sorted hierarchically
    let defs = vec![
        schema_for!(institution::Creation),
        schema_for!(institution::Query),
        schema_for!(institution::Query),
    ];

    fs::write(output, serde_json::to_string(&defs).unwrap())
        .expect("failed to write JSON schema to file");
}
