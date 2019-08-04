use glob::glob;
use std::env;
use std::path::Path;
use std::result::Result;

fn load_module(p: &Path) {
    println!("{}", p.display());
}

fn load_modules() {
    match env::var_os("FEOLA_SOURCE_PATH") {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let pattern = format!("{}/*.so", path.display());

                for entry in glob(&pattern).unwrap().filter_map(Result::ok) {
                    load_module(&entry);
                }
            }
        }
        None => println!("Empty source path."),
    }
}

fn main() {
    // Load modules
    load_modules();
    // Setup pipes for the frontends
    // Poll the pipes (or maybe there's a way to wait on events?)
    // Transmit the query from the pipe to all of the modules
    // Aggregate the results
    // Serialize the results to JSON
    // Send the results to the frontend that requested it
}
