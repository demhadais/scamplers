use std::process::Command;

fn main() {
    println!("cargo::rerun-if-changed=migrations");

    // Run migrations and generate lib.rs
    generate_schema();
}

fn generate_schema() {
    let mut diesel_cmd = Command::new("diesel");

    let args = ["migration", "run", "--migration-dir", "migrations"];
    diesel_cmd.args(args);

    let output = diesel_cmd.output().unwrap();

    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8(output.stderr).unwrap()
    );
}
