#[macro_use]
extern crate error_chain;
extern crate libc;
extern crate rustc_serialize;

use libc::c_char;
use rustc_serialize::json;
use std::ffi::CStr;
use std::ffi::CString;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        SystemTimeError(std::time::SystemTimeError);
    }
}


use std::fs::{self, DirEntry};
use std::path::Path;

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(f) = path.file_name() {
                    match f.to_str() {
                        Some("node_modules") => cb(&entry),
                        _ => visit_dirs(&path, cb)?,
                    };
                }
                ;
            }
        }
    }
    Ok(())
}

#[no_mangle]
pub extern "C" fn free_memory(pointer: *mut c_char) {
    unsafe {
        if pointer.is_null() {
            return;
        }
        CString::from_raw(pointer)
    };
}


#[no_mangle]
pub extern "C" fn get_dir(s: *const c_char) -> *mut c_char {
     let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };
    let r_str = c_str.to_str().unwrap();

    let mut res: Vec<String> = Vec::new(); // Result contain the path string.
    
    visit_dirs(r_str.as_ref(), &mut |entry: &DirEntry| { &res.push(entry.path().to_string_lossy().into_owned()); }).unwrap();
    
    let json_string = CString::new(json::encode(&res).unwrap()).unwrap();

    json_string.into_raw()
}
