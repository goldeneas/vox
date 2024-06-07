use std::process::Command;
use xtask_wasm::{anyhow::Result, clap, default_dist_dir};

#[derive(clap::Parser)]
enum Opt {
    Dist(xtask_wasm::Dist),
    Watch(xtask_wasm::Watch),
    Start(xtask_wasm::DevServer),
}


fn main() -> Result<()> {
    env_logger::init();

    let opt: Opt = clap::Parser::parse();

    match opt {
        Opt::Dist(dist) => {
            log::error!("Generating package...");

            dist
                .dist_dir_path("dist")
                .static_dir_path("vox-core/static")
                .app_name("vox-core")
                .run_in_workspace(true)
                .run("vox-core")?;
        }
        Opt::Watch(watch) => {
            log::error!("Watching for changes and check...");

            let mut command = Command::new("cargo");
            command.arg("check");

            watch.run(command)?;
        }
        Opt::Start(dev_server) => {
            log::error!("Starting the development server...");

            dev_server.arg("dist").start(default_dist_dir(false))?;
        }
    }

    Ok(())
}
