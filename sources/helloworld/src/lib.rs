use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn feola_init() {}

#[no_mangle]
pub extern "C" fn feola_search(query: &CStr) {
    println!("{:?}", query);
}
