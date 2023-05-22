use super::manifest;
use std::path::Path;

fn copy_contents(from: &Path, to: &Path) {
    let mut options = fs_extra::dir::CopyOptions::new();
    options.content_only = true;

    fs_extra::dir::create_all(&to, false).unwrap();
    fs_extra::dir::copy(&from, to, &options).unwrap();
}

pub fn save_current_version(snapshot_name: &str) {
    let source_folder = manifest::folder().join("src");
    let target_folder = manifest::folder().join("examples").join(snapshot_name);

    copy_contents(&source_folder, &target_folder);
}
