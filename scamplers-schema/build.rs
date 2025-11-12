use std::process::Command;

fn main() {
    let run_diesel_cli = option_env!("RUN_DIESEL_CLI");

    if let Some(run_diesel_cli) = run_diesel_cli {
        let run_diesel_cli: bool = run_diesel_cli.parse().unwrap();
        if !run_diesel_cli {
            return;
        }
    }

    println!("cargo::rerun-if-changed=migrations");

    // In case the db doesn't exist, run `diesel setup` to create it. However, if it
    // does exist, then it's a no-op. Either way, we want to reset it so we have a
    // fresh db before generating the Rust-side schema
    setup_db();
    reset_db();
    generate_schema();
}

fn setup_db() {
    let cmd = diesel_cmd(&["setup"]);

    run_command(cmd)
}

fn reset_db() {
    let cmd = diesel_cmd(&["database", "reset"]);

    run_command(cmd)
}

fn generate_schema() {
    let cmd = diesel_cmd(&["migration", "run"]);

    run_command(cmd)
}

fn run_command(mut cmd: Command) {
    let output = cmd.output().unwrap();

    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8(output.stderr).unwrap()
    );
}

fn diesel_cmd(args: &[&str]) -> Command {
    let mut cmd = Command::new("diesel");
    cmd.args(args);

    cmd
}
