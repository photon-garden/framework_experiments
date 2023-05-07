use crate::snapshot::manifest;
use crate::snapshot::Snapshot;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn clean_up(app: &nannou::prelude::App, snapshot: &Snapshot) {
    if !snapshot.did_capture_frames {
        return;
    }

    app.main_window().await_capture_frame_jobs().unwrap();

    fs::read_dir(images_folder_path())
        .unwrap()
        // Get the path for each entry.
        .map(|entry| entry.unwrap().path().to_str().unwrap().to_owned())
        // Only keep paths pointing to uncompressed images.
        .filter(|path| path.contains("uncompressed.tif"))
        .for_each(|uncompressed_path| clean_up_uncompressed_file(uncompressed_path, snapshot));
}

fn clean_up_uncompressed_file(uncompressed_path: String, snapshot: &Snapshot) {
    let compressed_path = uncompressed_path.replace(" uncompressed.tif", ".tif");

    // convert -compress lzw "image uncompressed.tif" image.tif
    let output = Command::new("convert")
        .arg("-compress")
        .arg("lzw")
        .arg(&uncompressed_path)
        .arg(&compressed_path)
        .output()
        .unwrap();

    if !output.status.success() {
        let error_message = String::from_utf8(output.stderr).unwrap();
        panic!("{}", error_message);
    }

    fs::remove_file(uncompressed_path).unwrap();

    save_image_placeholder_in_snapshot_directory(compressed_path, snapshot);
}

pub fn capture_frame(snapshot: &mut Snapshot, app: &nannou::prelude::App) {
    let image_path = uncompressed_path(snapshot.image_name());
    app.main_window().capture_frame(image_path);
}

fn snapshot_image_name_from_compressed_path(compressed_path_string: &str) -> String {
    compressed_path_string
        .replace(images_folder_path().join("").to_str().unwrap(), "")
        .replace(".tif", "")
        .trim()
        .to_string()
}

// In our snapshot directory, we make an empty file with the same name as the image.
// This makes it easy to use VS Code's command + p feature to find the image we're
// interested in.
pub fn save_image_placeholder_in_snapshot_directory(compressed_path: String, snapshot: &Snapshot) {
    let snapshot_image_name = snapshot_image_name_from_compressed_path(&compressed_path);
    let path = manifest::folder()
        .join("examples")
        .join(snapshot.source_code_folder_name.clone())
        .join(snapshot_image_name);
    std::fs::write(path, "").unwrap();
}

// fn compressed_path(snapshot_name: String) -> PathBuf {
//     path(&compressed_name(snapshot_name))
// }

fn uncompressed_path(snapshot_name: String) -> PathBuf {
    path(&uncompressed_name(snapshot_name))
}

fn path(name: &str) -> PathBuf {
    images_folder_path().join(name)
}

pub fn images_folder_path() -> PathBuf {
    manifest::folder().join("images")
}

fn compressed_name(snapshot_name: String) -> String {
    format!("{}{}", snapshot_name, ".tif")
}

fn uncompressed_name(snapshot_name: String) -> String {
    format!("{} uncompressed{}", snapshot_name, ".tif")
}
