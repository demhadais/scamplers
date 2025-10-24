use std::process::Command;

fn main() {
    println!("cargo::rerun-if-changed=migrations");

    for cmd in [reset_db(), generate_schema()] {
        run_command(cmd);
    }
}

fn diesel_cmd(args: &[&str]) -> Command {
    let mut cmd = Command::new("diesel");
    cmd.args(args);

    cmd
}

fn reset_db() -> Command {
    let cmd = diesel_cmd(&["database", "reset"]);

    cmd
}

fn generate_schema() -> Command {
    let cmd = diesel_cmd(&["migration", "run"]);

    cmd
}

fn run_command(mut cmd: Command) {
    let output = cmd.output().unwrap();

    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8(output.stderr).unwrap()
    );
}
