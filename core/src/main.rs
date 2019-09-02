use glob::glob;
use libloading::{Library, Symbol};
use std::env;
use std::ffi::CString;
use std::result::Result;

struct Source {
    lib: Box<Library>,
}

impl Source {
    fn search(&self, string: String) {
        let func: Symbol<fn(CString)>;
        unsafe {
            func = self.lib.get(b"feola_search").unwrap();
        }
        func(CString::new(string).unwrap());
    }
}

/// Load all the sources available in the paths.
///
/// The paths are at the moment specified using the `FEOLA_SOURCE_PATH`
/// environment variable.
fn load_sources(mut sources: &mut Vec<Source>) {
    match env::var_os("FEOLA_SOURCE_PATH") {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let pattern = format!("{}/*.so", path.display());

                for entry in glob(&pattern).unwrap().filter_map(Result::ok) {
                    let lib = Box::new(Library::new(&entry).unwrap());
                    sources.push(Source { lib: lib })
                }
            }
        }
        None => println!("Empty source path."),
    }
}

fn main() {
    // Load sources
    let sources: Vec<Source> = vec![];
    load_sources(&mut sources);
    // test code
    for source in &sources {
        source.search("Test".to_string());
    }

    //TODO
    // Setup pipes for the frontends
    // Poll the pipes (or maybe there's a way to wait on events?)
    // Transmit the query from the pipe to all of the sources
    // Aggregate the results
    // Serialize the results to JSON
    // Send the results to the frontend that requested it
}
