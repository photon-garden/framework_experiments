use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub struct Seed {
    pub value: u64,
    save_path: Option<PathBuf>,
}

impl Seed {
    pub fn load() -> Seed {
        let path = Path::new(file!())
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("seed");

        let save_path;

        let value = if path.exists() {
            // File already exists, so no need
            // to save it again.
            save_path = None;
            Seed::get_value_from_file(&path)
        } else {
            // File needs to be created, so
            // specify a save path.
            save_path = Some(path);
            Seed::get_value_from_current_time()
        };

        Seed { value, save_path }
    }

    pub fn save_to_file(&self) {
        // We computed the seed from the current time,
        // so now it needs to be saved to a file so
        // future runs are deterministic.
        if let Some(path) = &self.save_path {
            fs::write(path, self.value.to_string()).unwrap();
        }
    }

    pub fn clean_up_file(&self) {
        // The seed file is already saved in the
        // appropriate snapshots/ subfolder, together
        // with our source code.
        //
        // We delete it here so that we get a new seed
        // next time we compile.
        if let Some(path) = &self.save_path {
            fs::remove_file(path).unwrap();
        }
    }

    pub fn get_value_from_current_time() -> u64 {
        chrono::Local::now().timestamp_nanos() as u64
    }

    pub fn get_value_from_file(path: &Path) -> u64 {
        let value = fs::read_to_string(path).unwrap();
        let value = value.trim();
        value.parse::<u64>().unwrap()
    }
}
