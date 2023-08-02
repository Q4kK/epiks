use dirs::home_dir;
use std::fs;
use std::io;
use std::path::*;

fn main() {
    let mut input = "";
    let home = match home_dir() {
        Some(home) => home,
        None => {
            panic!("No home directory found. oopsie daisy uh oh spaghettio");
        }
    };
    println!("Enter a directory in which the project will go. (Default: ~/projects/)");
    if io::stdin().read_line(&mut input).unwrap() = "" {
        mkdir(&home.join("/projects/")).unwrap();
    } else {
        let path = Path::new(&input).to_path_buf();
        mkdir()
    }
}

fn mkdir(path: &Path) -> Result<(), io::Error> {
    println!("Building directory now.");
    fs::DirBuilder::new().recursive(true).create(path)
}
