use rustup_configurator::target::Target;

fn main() {
    // Get a list of all targets and if they are installed
    let list: Vec<Target> = rustup_configurator::target::list().unwrap();
    println!("All targets: \n{:#?}", list);

    // Get all installed targets
    let installed: Vec<Target> = rustup_configurator::target::installed().unwrap();
    println!("Installed targets: \n{:#?}", installed);

    // Install some targets
    rustup_configurator::target::install(&["aarch64-apple-ios".into()]).unwrap();
}
