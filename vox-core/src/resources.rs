use crate::Texture;

// TODO: this function will not work with WASM!
// https://sotrh.github.io/learn-wgpu/beginner/tutorial9-models/#accessing-files-from-wasm
pub fn load_binary(file_name: &str) -> anyhow::Result<Vec<u8>> {
    let path = std::path::Path::new(env!("OUT_DIR"))
        .join("res")
        .join(file_name);

    let data = std::fs::read(path)?;
    Ok(data)
}

pub fn load_texture(file_name: &str) -> anyhow::Result<Texture> {

}
