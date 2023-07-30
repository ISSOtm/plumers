use std::{path::Path, process::Command};

use cargo_metadata::MetadataCommand;
use eyre::{ensure, eyre, Context};

pub(super) fn update_libplum(mut args: std::env::Args) -> Result<(), eyre::Report> {
    ensure!(
        args.next().is_none(),
        "This xtask does not take any arguments!"
    );

    let metadata = MetadataCommand::new()
        .no_deps()
        .exec()
        .wrap_err("Failed to parse Cargo metadata")?;
    let libplum_sys = metadata
        .root_package()
        .expect("libplum-sys is not the root package!?");
    let wrapper_version = &libplum_sys.version;
    let libplum_tag = &wrapper_version.build;
    let crate_root = {
        let mut workspace_root = metadata
            .workspace_root
            .canonicalize()
            .wrap_err("Failed to canonicalize path to workspace root")?;
        workspace_root.push("libplum-sys");
        workspace_root
    };

    eprintln!("Starting import of libplum {libplum_tag}...");

    let libplum_src_dir = {
        let mut path = crate_root.clone();
        path.push("libplum");
        path.push(""); // We want a directory, not a file!
        path
    };
    let output_dir = {
        let mut path = crate_root.clone();
        path.push("src");
        path.push(""); // We want a directory, not a file!
        path
    };

    get_libplum_sources(&crate_root, &libplum_src_dir, libplum_tag.as_str())?;
    create_basefiles(&libplum_src_dir)?;
    move_basefiles(&libplum_src_dir, &output_dir)?;

    eprintln!("Done!");

    Ok(())
}

fn get_libplum_sources(
    crate_root: &Path,
    libplum_src_dir: &Path,
    tag: &str,
) -> Result<(), eyre::Report> {
    if libplum_src_dir
        .try_exists()
        .wrap_err("Failed to check whether libplum src dir exists")?
    {
        eprintln!("OK: {} exists", libplum_src_dir.display());
    } else {
        eprintln!(
            "{} doesn't exist, cloning repo...",
            libplum_src_dir.display()
        );
        let clone_status = Command::new("git")
            .args([
                "clone",
                "-q",
                "https://github.com/aaaaaa123456789/libplum.git",
            ])
            .current_dir(crate_root)
            .status()
            .wrap_err("Failed to execute git clone!")?;
        ensure!(clone_status.success(), "git clone failed");
    }

    let checkout_status = Command::new("git")
        .args(["checkout", "-q", "-f", tag, "--"])
        .current_dir(libplum_src_dir)
        .status()
        .wrap_err("Failed to execute git checkout!")?;
    ensure!(checkout_status.success(), "git checkout failed");

    Ok(())
}

fn create_basefiles(libplum_src_dir: &Path) -> Result<(), eyre::Report> {
    eprintln!("Creating basefiles...");

    let make_status = Command::new("make")
        .args(["-s", "basefiles"])
        .current_dir(libplum_src_dir)
        .status()
        .wrap_err("Failed to execute make!")?;
    ensure!(make_status.success(), "make basefiles failed");

    Ok(())
}

fn move_basefiles(libplum_src_dir: &Path, target_dir: &Path) -> Result<(), eyre::Report> {
    eprintln!("Moving basefiles...");

    let build_dir = libplum_src_dir.join("build");
    let move_file = |file_name| {
        let from = build_dir.join(file_name);
        let to = target_dir.join(file_name);
        std::fs::rename(&from, &to).wrap_err(eyre!(
            "Failed to move {} to {}",
            from.display(),
            to.display()
        ))
    };

    move_file("libplum.c")?;
    move_file("libplum.h")
}
