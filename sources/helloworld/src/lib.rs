use std::ffi::CStr;

#[no_mangle]
pub extern fn feola_search(query: &CStr) {
    println!("{:?}", query);
}
