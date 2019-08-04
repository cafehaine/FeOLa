use glob::glob;
use std::env;
use std::path::Path;
use std::result::Result;

use libloading::{Library, Symbol};

/// Load a single source, given it's path.
fn load_source(p: &Path) {
    let lib = Library::new(p).unwrap();

    unsafe {
        let func: Symbol< fn() > = lib.get(b"feola_search").unwrap();

        func();
    }
}

/// Load all the sources available in the paths.
///
/// The paths are at the moment specified using the `FEOLA_SOURCE_PATH`
/// environment variable.
fn load_sources() {
    match env::var_os("FEOLA_SOURCE_PATH") {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let pattern = format!("{}/*.so", path.display());

                for entry in glob(&pattern).unwrap().filter_map(Result::ok) {
                    load_source(&entry);
                }
            }
        }
        None => println!("Empty source path."),
    }
}

fn main() {
    // Load sources
    load_sources();
    // Setup pipes for the frontends
    // Poll the pipes (or maybe there's a way to wait on events?)
    // Transmit the query from the pipe to all of the sources
    // Aggregate the results
    // Serialize the results to JSON
    // Send the results to the frontend that requested it
}
