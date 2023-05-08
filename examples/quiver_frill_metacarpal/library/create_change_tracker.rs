use crate::prelude::*;

pub fn create_change_tracker() -> impl FnMut(f32) -> bool {
    let mut maybe_previous_value: Option<f32> = None;

    move |new_value| {
        let has_changed = match maybe_previous_value {
            Some(previous_value) => previous_value.far_from(new_value),
            None => true,
        };

        maybe_previous_value = Some(new_value);

        has_changed
    }
}
