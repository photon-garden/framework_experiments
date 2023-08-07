use crate::snapshot::manifest;
use duct::cmd;
use std::io::{BufRead, BufReader};
use std::{fs, fs::DirEntry, io::ErrorKind, time::SystemTime};

use itertools::Itertools;

pub fn create() {
    let temp_folder = manifest::folder().join("temp");
    if let Err(error) = fs::remove_dir_all(temp_folder.clone()) {
        match error.kind() {
            ErrorKind::NotFound => {
                // No problem. Keep going.
            }
            _ => {
                println!("{error}");
                panic!("Error removing temp folder.");
            }
        }
    };
    fs::create_dir(temp_folder.clone()).expect("Couldn't create the temp folder.");

    let path_to_images_folder = super::image::images_folder_path();

    let mut image_path_dir_entries = fs::read_dir(path_to_images_folder)
        .expect("Couldn't find images folder. Does it exist?")
        .filter(|file_path| !is_ds_store(file_path.as_ref().unwrap()))
        .collect_vec();

    image_path_dir_entries.sort_by(|a, b| {
        let a_created_time = get_created_time(a.as_ref().unwrap());
        let b_created_time = get_created_time(b.as_ref().unwrap());
        a_created_time.cmp(&b_created_time)
    });

    for (index, dir_entry) in image_path_dir_entries.into_iter().enumerate() {
        let padded_index = pad_number(index);
        let new_name = format!("{padded_index}.tif");
        let original_path = dir_entry.unwrap().path();
        let new_path = temp_folder.join(new_name);
        fs::copy(original_path, new_path).expect("Couldn't copy image file.");
    }

    let temp_images_path = temp_folder.join("*.tif");

    let artwork_name = manifest::name();
    let video_file_name = format!("{artwork_name}.mp4");
    let video_path = temp_folder.join(video_file_name);
    let video_path_string = video_path.to_str().unwrap();

    // Helpful info on ffmpeg commands here:
    // https://unix.stackexchange.com/questions/28803/how-can-i-reduce-a-videos-size-with-ffmpeg
    let reader = cmd!(
        "ffmpeg",
        "-framerate",
        "15", // 15 fps
        "-pattern_type",
        "glob",
        "-i",
        temp_images_path, // All images in the temp folder.
        "-vcodec",
        "libx264", // Compress using the H.265 codec.
        "-crf",
        "28", // Compression level. Higher is more compressed.
        "-pix_fmt",
        "yuv420p",         // "Good for compatibility."
        video_path_string  // Output file path.
    )
    .stderr_to_stdout()
    .reader()
    .unwrap();

    let lines = BufReader::new(reader).lines();

    for maybe_line in lines {
        let line = maybe_line.unwrap();
        println!("{line}");
    }

    // Open the video we just created.
    cmd!("open", video_path).run().unwrap();
}

fn get_created_time(dir_entry: &DirEntry) -> SystemTime {
    dir_entry.metadata().unwrap().created().unwrap()
}

fn is_ds_store(dir_entry: &DirEntry) -> bool {
    dir_entry.file_name().to_str().unwrap().contains("DS_Store")
}

// Pads a number with 5 zeros. Good for up to 99,999 values.
fn pad_number(number: usize) -> String {
    format!("{:05}", number)
}
