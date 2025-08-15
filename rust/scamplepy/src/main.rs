use std::{fs, io, path::Path, str::FromStr};

use anyhow::ensure;
use camino::{Utf8Path, Utf8PathBuf};
use scamplers::{
    COMMON_SUBMODULE_NAME, CREATE_SUBMODULE_NAME, ERRORS_SUBMODULE_NAME, QUERY_SUBMODULE_NAME,
    RESPONSES_SUBMODULE_NAME, UPDATE_SUBMODULE_NAME,
};

fn generate_typestubs() -> anyhow::Result<()> {
    let stub_info = scamplepy::stub_info()?;
    stub_info.generate()?;

    Ok(())
}

fn create_dir_exists_ok<P>(path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let result = fs::create_dir(path);

    match result {
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => Ok(()),
        _ => result,
    }
}

fn correct_typestubs() -> anyhow::Result<()> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let typestub_dir = Utf8PathBuf::from_str(manifest_dir)?;

    let submodules = [
        CREATE_SUBMODULE_NAME,
        QUERY_SUBMODULE_NAME,
        UPDATE_SUBMODULE_NAME,
        ERRORS_SUBMODULE_NAME,
        COMMON_SUBMODULE_NAME,
        // RESPONSES_SUBMODULE_NAME,
    ]
    .map(|s| Utf8PathBuf::from_str(&s.replace('.', "/")).unwrap())
    .map(|d| typestub_dir.join(d));

    for submodule in submodules {
        create_dir_exists_ok(&submodule)?;

        fs::write(
            submodule.join("__init__.py"),
            format!(
                "from scamplepy.{} import * # noqa: F403\n",
                submodule.file_name().unwrap()
            ),
        )?;

        let original_typestub_file =
            submodule.with_file_name(format!("{}.pyi", submodule.file_name().unwrap()));

        fs::rename(original_typestub_file, submodule.join("__init__.pyi"))?;
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    generate_typestubs()?;
    correct_typestubs()?;

    Ok(())
}
