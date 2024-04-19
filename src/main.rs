use std::env;

mod actions;
mod managers;
mod support;
mod cli;

type Structure = support::prelude::Structure;

#[tokio::main]
async fn main() {
    let platform = env::consts::OS.to_string();
    let arch = env::consts::ARCH.to_string();

    let my_system = Structure::new(platform, arch);

    dbg!(&my_system);

    let args = env::args().collect::<Vec<_>>();

    dbg!(&args);

    println!("Hello, world!");
}
