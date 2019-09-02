use glob::glob;
use libloading::{Library, Symbol};
use std::env;
use std::ffi::CString;
use std::result::Result;

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
                    let lib = Library::new(&entry).unwrap();
                    let func: Symbol<fn(CString)>;
                    unsafe {
                        func = lib.get(b"feola_search").unwrap();
                    }
                    func(CString::new("Hello").unwrap());
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
