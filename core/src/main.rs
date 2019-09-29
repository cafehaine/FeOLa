use glob::glob;
use libloading::{Library, Symbol};
use std::env;
use std::thread;
use std::ffi::CString;
use std::os::unix::net::{UnixListener, UnixStream};
use std::result::Result;
#[macro_use] extern crate log;

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

    fn init(&self) {
        let func: Symbol<fn()>;
        unsafe {
            func = self.lib.get(b"feola_init").unwrap();
        }
        func();
    }

}

/// Load all the sources available in the paths.
///
/// The paths are at the moment specified using the `FEOLA_SOURCE_PATH`
/// environment variable.
fn load_sources(sources: &mut Vec<Source>) {
    match env::var_os("FEOLA_SOURCE_PATH") {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let pattern = format!("{}/*.so", path.display());

                for entry in glob(&pattern).unwrap().filter_map(Result::ok) {
                    debug!("Loading source {:?}", entry);
                    let lib = Box::new(Library::new(&entry).unwrap());
                    let source = Source { lib: lib};
                    source.init();
                    sources.push(source)
                }
            }
        }
        None => println!("Empty source path."),
    }
}

fn handle_frontend(stream: UnixStream) {
    //TODO
}

fn main() {
    env_logger::init();

    extern crate xdg;
    let xdg_dirs = xdg::BaseDirectories::with_prefix("feola").unwrap();

    // Load sources
    info!("Loading sources");
    let mut sources: Vec<Source> = vec![];
    load_sources(&mut sources);

    info!("Setting up UNIX socket");
    let socket_path = xdg_dirs
        .place_runtime_file("socket")
        .expect("Cannot create runtime directory");

    // Will break if feola was already ran since the system last boot, try to
    // delete the previous socket first?
    let listener = UnixListener::bind(socket_path).expect("Cannot bind socket");
    //TODO
    info!("Awaiting frontends");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                debug!("New frontend");
                thread::spawn(|| handle_frontend(stream));
            }
            Err(err) => {
                warn!("Could not connect to frontend.")
            }
        }
    }
    // Setup pipes for the frontends
    // Poll the pipes (or maybe there's a way to wait on events?)
    // Transmit the query from the pipe to all of the sources
    // Stream the results to the frontend (how?)
}
