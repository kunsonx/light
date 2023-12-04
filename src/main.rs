#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CString;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    unsafe {
        let c_string = CString::new("rust").expect("CString::new failed");
        add(1, 2, c_string.into_raw());
    }
    println!("Hello, world!");
}
