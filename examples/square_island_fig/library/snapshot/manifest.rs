use std::path::Path;

pub fn folder() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

pub fn name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
