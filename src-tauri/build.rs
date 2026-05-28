use std::fs;
use std::io;
use std::path::Path;

#[cfg(windows)]
use std::process::Command;

fn main() {
    if let Err(error) = prepare_generated_artifact_path() {
        panic!("failed to prepare generated artifact path under build/: {error}");
    }

    tauri_build::build();

    if let Err(error) = cleanup_generated_artifact_link() {
        panic!("failed to cleanup generated artifact link at src-tauri/gen: {error}");
    }
}

fn prepare_generated_artifact_path() -> io::Result<()> {
    let link_path = Path::new("gen");
    let target_path = Path::new("..").join("build").join("tauri").join("gen");
    fs::create_dir_all(&target_path)?;

    if let Ok(metadata) = fs::symlink_metadata(link_path) {
        if metadata.file_type().is_symlink() {
            return Ok(());
        }

        if metadata.is_dir() {
            copy_dir_contents(link_path, &target_path)?;
            fs::remove_dir_all(link_path)?;
        } else {
            fs::remove_file(link_path)?;
        }
    }

    #[cfg(windows)]
    {
        create_windows_junction(link_path, &target_path)?;
    }

    #[cfg(not(windows))]
    {
        std::os::unix::fs::symlink(&target_path, link_path)?;
    }

    Ok(())
}

fn copy_dir_contents(source: &Path, destination: &Path) -> io::Result<()> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_contents(&source_path, &destination_path)?;
        } else {
            fs::copy(&source_path, &destination_path)?;
        }
    }
    Ok(())
}

fn cleanup_generated_artifact_link() -> io::Result<()> {
    let link_path = Path::new("gen");
    if !link_path.exists() {
        return Ok(());
    }

    let metadata = fs::symlink_metadata(link_path)?;
    if metadata.file_type().is_symlink() {
        fs::remove_dir(link_path)?;
    }

    Ok(())
}

#[cfg(windows)]
fn create_windows_junction(link_path: &Path, target_path: &Path) -> io::Result<()> {
    let target = target_path
        .canonicalize()?
        .into_os_string()
        .into_string()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "non-utf8 target path"))?;
    let link = link_path
        .to_path_buf()
        .into_os_string()
        .into_string()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "non-utf8 link path"))?;

    let status = Command::new("cmd")
        .args(["/C", "mklink", "/J", &link, &target])
        .status()?;
    if status.success() {
        return Ok(());
    }

    Err(io::Error::new(
        io::ErrorKind::PermissionDenied,
        "failed to create junction at src-tauri/gen",
    ))
}
