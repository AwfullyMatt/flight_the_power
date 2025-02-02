use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the root directory of your project (where Cargo.toml resides)
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    // Determine if this is a release build
    let profile = env::var("PROFILE").unwrap(); // "debug" or "release"

    // Path to the target directory (e.g., target/release/)
    let target_dir = manifest_dir.join("target").join(profile);

    // Path to the assets directory in the target folder
    let assets_dir = target_dir.join("assets");

    // Path to the ron directory in the target folder
    let ron_dir = target_dir.join("ron");

    // Create the target/assets directory if it doesn't exist
    if !assets_dir.exists() {
        fs::create_dir_all(&assets_dir).expect("Failed to create assets directory");
    }

    // Create the target/ron directory if it doesn't exist
    if !ron_dir.exists() {
        fs::create_dir_all(&ron_dir).expect("Failed to create ron directory");

        // Copy all files from your project's assets/ to target/release/assets/
        copy_dir_all(manifest_dir.join("assets"), &assets_dir).unwrap();
        // Copy all files from your project's ron/ to target/release/assets/
        copy_dir_all(manifest_dir.join("ron"), &ron_dir).unwrap();
    }

    fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
        fs::create_dir_all(&dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            if ty.is_dir() {
                copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            }
        }
        Ok(())
    }
}
