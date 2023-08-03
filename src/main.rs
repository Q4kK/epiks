use dirs::home_dir;
use std::fs;
use std::io;
use std::path::*;

fn main() {
    let home = match home_dir() {
        Some(home) => home,
        None => {
            panic!("No home directory found. oopsie daisy uh oh spaghettio");
        }
    };
    let srcpath = home.join("projects/src/");
    let binpath = home.join("projects/bin/");
    println!("Project building in {:?}, {:?}", &srcpath, &binpath);
    mkdir(&srcpath).unwrap();
    mkdir(&binpath).unwrap();
}

fn mkdir(path: &Path) -> Result<(), io::Error> {
    println!("Building directory now.");
    fs::DirBuilder::new().recursive(true).create(path)
}

// fn make_project(proj_name: String) {
//     srcpath = ""
//     fs::File::create("{srcpath}/main");
// }
