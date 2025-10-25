use std::{fs, path::PathBuf};

use clap::Parser;
use heck::ToSnekCase;
use scamplers_models::{institution, person};
use schemars::schema_for;

#[derive(clap::Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long, default_value = "json-schema")]
    output_dir: String,
}

fn main() {
    let Cli { output_dir } = Cli::parse();
    // These are sorted hierarchically
    let defs = vec![
        schema_for!(institution::Creation),
        schema_for!(institution::Query),
        schema_for!(institution::Institution),
        schema_for!(person::Creation),
        schema_for!(person::Query),
        schema_for!(person::PersonSummary),
        schema_for!(person::Person),
    ];

    let output_dir = PathBuf::from(output_dir);
    if !output_dir.exists() {
        fs::create_dir(&output_dir).unwrap_or_else(|_| {
            panic!(
                "directory {} should be writable",
                output_dir.to_str().unwrap()
            )
        });
    }

    for def in defs {
        let filename = def
            .get("title")
            .expect("schema should have 'title' key")
            .as_str()
            .unwrap()
            .to_snek_case();
        let path = output_dir.join(filename).with_extension("json");

        fs::write(&path, serde_json::to_string(&def).unwrap()).unwrap_or_else(|_| {
            panic!(
                "JSON schema should be writable to {}",
                path.to_str().unwrap()
            )
        });
    }
}
