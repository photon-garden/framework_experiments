mod friendly_words;
mod image;
mod manifest;
pub mod rand;
pub mod seed;
mod source_code;
pub mod timelapse;

use rand::Rand;
use seed::Seed;

pub struct Snapshot {
    pub did_capture_frames: bool,
    pub seed: u64,
    pub source_code_folder_name: String,
    frame_number: u64,
    has_used_source_code_folder_name_as_image_name: bool,
    image_name_randomizer: Rand,
}

impl Snapshot {
    fn create() -> Snapshot {
        let seed = Seed::load();
        seed.save_to_file();

        // Always use the current time as the seed for generating names. If we used
        // the seed saved in the file, this would try to generate the same name as
        // the original file, and error when it tried to overwrite it. Instead, we
        // just generate a new name every time.
        let image_name_randomizer = Rand::from_seed(Seed::get_value_from_current_time());

        let new_snapshot = Snapshot {
            seed: seed.value,
            did_capture_frames: false,
            frame_number: 0,
            source_code_folder_name: friendly_name(&image_name_randomizer),
            has_used_source_code_folder_name_as_image_name: false,
            image_name_randomizer,
        };

        // If this is already saved in the examples folder, no need to save another copy.
        if !file!().contains("examples") {
            source_code::save_current_version(&new_snapshot.source_code_folder_name);
        }

        seed.clean_up_file();

        new_snapshot
    }

    fn image_name(&mut self) -> String {
        if self.has_used_source_code_folder_name_as_image_name {
            friendly_name(&self.image_name_randomizer)
        } else {
            self.has_used_source_code_folder_name_as_image_name = true;
            self.source_code_folder_name.clone()
        }
    }

    pub fn get_rand(&self) -> Rand {
        Rand::from_seed(self.seed)
    }

    pub fn capture_frame(&mut self, app: &nannou::prelude::App) {
        image::capture_frame(self, app);

        self.did_capture_frames = true;
        self.frame_number += 1;
    }
}

pub fn save() -> Snapshot {
    Snapshot::create()
}

pub fn exit(app: &nannou::prelude::App, model: crate::prelude::Model) {
    if model.snapshot.did_capture_frames {
        image::clean_up(app, &model.snapshot);
    }
}

fn friendly_name(rand: &Rand) -> String {
    let predicates = friendly_words::predicates();
    let objects = friendly_words::objects();

    let first_predicate = rand.element(&predicates);
    let second_predicate = rand.element(&predicates);

    let object = rand.element(&objects);

    format!("{}_{}_{}", first_predicate, second_predicate, object)
}
