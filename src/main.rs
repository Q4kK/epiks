use dirs::home_dir;
use std::fs;
use std::io;
use std::path::*;

fn main() {
    let paths = ["src/", "bin/"];
    let mut buffer = String::new();
    let stdin = io::stdin();
    let home = match home_dir() {
        Some(home) => home.join("projects/"),
        None => {
            panic!("No home directory found. oopsie daisy uh oh spaghettio");
        }
    };
    println!("Hey, you should name the project:");
    stdin.read_line(&mut buffer).unwrap();
    let project_path = home.join(buffer.trim());
    println!("Building Main.java in {}/src/", project_path.display());
    for subfolder in paths {
        make_directory(&project_path.join(subfolder)).unwrap();
    }
    makeproject(&project_path.join("src/"));
}

fn make_directory(path: &Path) -> Result<(), io::Error> {
    fs::DirBuilder::new().recursive(true).create(path)
}

fn makeproject(path: &Path) {
    fs::write(
        path.join("Main.java"),
        "class Main {\n \n public static void main(String[] args) {\n \n} \n}",
    )
    .ok();
}
