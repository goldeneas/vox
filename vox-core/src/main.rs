use vox_core::run;

fn main() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            wasm_bindgen_futures::spawn_local(run());
        } else {
            pollster::block_on(run());
        }
    }
}
