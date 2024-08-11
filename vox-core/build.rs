use anyhow::*;
use std::path::*;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::env;

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string).join("target").join(build_type);
    println!("cargo:rustc-env=VOX_OUTPUT_DIR={}", path.to_str().unwrap());
    path
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=res/*");

    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    let paths_to_copy = vec!["res/"];
    copy_items(&paths_to_copy, get_output_path(), &copy_options)?;

    Ok(())
}
