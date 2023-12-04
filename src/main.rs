#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CString;
use std::os::raw::c_char;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    unsafe {
        let c_string = CString::new("rust").expect("CString::new failed");
        add(1, 2, c_string.into_raw() as *mut c_char);
    }
    println!("Hello, world!");
}
