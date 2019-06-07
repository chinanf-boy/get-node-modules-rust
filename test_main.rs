#[macro_use]
extern crate error_chain;

use std::env;

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

fn run() -> Result<()> {
    let current_dir = env::current_dir()?;
    let mut v = Vec::new();
    visit_dirs(&current_dir, &mut |entry: &DirEntry| { &v.push(entry.path()); }).unwrap();
    println!("{:?}", v);
    Ok(())
}

quick_main!(run);
