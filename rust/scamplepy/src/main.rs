use std::{fs, io, str::FromStr};

use anyhow::Context;
use camino::{Utf8Path, Utf8PathBuf};
use scamplers::{
    COMMON_SUBMODULE_NAME, CREATE_SUBMODULE_NAME, ERRORS_SUBMODULE_NAME, PARENT_MODULE_NAME,
    QUERY_SUBMODULE_NAME, RESPONSES_SUBMODULE_NAME, UPDATE_SUBMODULE_NAME,
};

fn generate_typestubs() -> anyhow::Result<()> {
    let stub_info = scamplepy::stub_info()?;
    stub_info.generate()?;

    Ok(())
}

fn create_dir_exists_ok(path: &Utf8Path) -> io::Result<()> {
    let result = fs::create_dir(path);

    match result {
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => Ok(()),
        _ => result,
    }
}

fn module_name_to_path(typestub_dir: &Utf8Path, module_name: &str) -> Utf8PathBuf {
    typestub_dir.join(Utf8PathBuf::from_str(&module_name.replace('.', "/")).unwrap())
}

fn add_wildcard_imports(path: &Utf8Path, imports: &[&str]) -> anyhow::Result<()> {
    let original_contents = fs::read_to_string(path)?;
    let imports = imports
        .iter()
        .map(|s| format!("from {s} import * # noqa:403"))
        .reduce(|s1, s2| format!("{s1}\n{s2}"))
        .unwrap_or_default();
    let new_contents = format!("{imports}\n{original_contents}");

    fs::write(path, new_contents).context(format!("failed to read {path}"))?;

    Ok(())
}

fn create_initdotpy(submodule_path: &Utf8Path, imports: &[&str]) -> anyhow::Result<()> {
    let initdotpy_path = submodule_path.join("__init__.py");
    fs::write(&initdotpy_path, "")?;

    add_wildcard_imports(&initdotpy_path, imports).context(format!(
        "failed to add wildcard imports to {initdotpy_path}"
    ))?;

    Ok(())
}

fn move_submodule_typestub_file(submodule_path: &Utf8Path) -> anyhow::Result<Utf8PathBuf> {
    let original_typestub_filepath =
        submodule_path.with_file_name(format!("{}.pyi", submodule_path.file_name().unwrap()));

    let new_typestub_filepath = submodule_path.join("__init__.pyi");

    fs::rename(original_typestub_filepath, &new_typestub_filepath)?;

    Ok(new_typestub_filepath)
}

fn correct_submodule_typestubs(
    typestub_dir: &Utf8Path,
    submodule_name: &str,
) -> anyhow::Result<()> {
    let submodule_path = module_name_to_path(typestub_dir, submodule_name);

    create_dir_exists_ok(&submodule_path)
        .context(format!("failed to creat directory {submodule_path}"))?;

    create_initdotpy(&submodule_path, &[submodule_name]).context(format!(
        "failed to create __init__.py for {submodule_name} in {typestub_dir}"
    ))?;

    let new_typestub_filepath = move_submodule_typestub_file(&submodule_path)
        .context(format!("failed to move typestub file for {submodule_path}"))?;
    if submodule_name == COMMON_SUBMODULE_NAME {
        return Ok(());
    }

    add_wildcard_imports(&new_typestub_filepath, &[submodule_name, "..common"]).context(
        format!("failed to add wildcared imports to {new_typestub_filepath}"),
    )?;

    Ok(())
}

fn correct_parent_module_typestubs(
    typestub_dir: &Utf8Path,
    parent_module_name: &str,
) -> anyhow::Result<()> {
    let module_path = module_name_to_path(typestub_dir, parent_module_name);

    create_initdotpy(&module_path, &[parent_module_name]).context(format!(
        "failed to create __init__.py for {parent_module_name} in {module_path}"
    ))?;

    let typestub_filepath = typestub_dir.join(parent_module_name).join("__init__.pyi");

    add_wildcard_imports(
        &typestub_filepath,
        &[
            CREATE_SUBMODULE_NAME,
            QUERY_SUBMODULE_NAME,
            UPDATE_SUBMODULE_NAME,
            ERRORS_SUBMODULE_NAME,
            RESPONSES_SUBMODULE_NAME,
        ],
    )
    .context(format!(
        "failed to add wildcared imports to {typestub_filepath}"
    ))?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    generate_typestubs().context("failed to generate typestubs")?;

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let typestub_dir = Utf8PathBuf::from_str(manifest_dir)?;

    let submodule_names = [
        CREATE_SUBMODULE_NAME,
        QUERY_SUBMODULE_NAME,
        UPDATE_SUBMODULE_NAME,
        ERRORS_SUBMODULE_NAME,
        COMMON_SUBMODULE_NAME,
        RESPONSES_SUBMODULE_NAME,
    ];

    for submodule_name in submodule_names {
        correct_submodule_typestubs(&typestub_dir, submodule_name)
            .context("failed to correct submodule typestubs")?;
    }

    correct_parent_module_typestubs(&typestub_dir, PARENT_MODULE_NAME)
        .context("failed to correct parent module typestubs")?;

    Ok(())
}
