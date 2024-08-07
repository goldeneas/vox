use std::{ffi::OsStr, path::Path};

// FIXME: This function will not work with WASM!
// https://sotrh.github.io/learn-wgpu/beginner/tutorial9-models/#accessing-files-from-wasm
pub fn load_binary(file_name: &str) -> anyhow::Result<Vec<u8>> {
    let path = std::path::Path::new(env!("VOX_OUTPUT_DIR"))
        .join("res")
        .join(file_name);

    let data = std::fs::read(path)?;
    Ok(data)
}

pub fn get_extension(file_name: &str) -> Option<&str> {
    Path::new(file_name)
        .extension()
        .and_then(OsStr::to_str)
}
