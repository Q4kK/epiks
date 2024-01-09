use std::fs;
use std::io;
use std::path;
use std::path::*;
use std::process::Command;

fn main() {
    let path = Path::new("./");
    let mut input = String::new();
    let stdin = io::stdin();
    println!("Hey, you should name the project:");
    stdin.read_line(&mut input).unwrap();
    let project_path = path.join(input.trim());
    let project_path_src = &project_path.join("src/");

    check_for_dupe_dir(&project_path);
    make_directory(&project_path).unwrap(); //make project path
    make_directory(&project_path_src).unwrap(); //make project path with a src/ folder
    makeproject(&project_path_src); //make Main.java in src/ folder
}
fn make_directory(path: &Path) -> Result<(), io::Error> {
    fs::DirBuilder::new().recursive(true).create(path)
}

fn makeproject(path: &Path) {
    fs::write(
        path.join("Main.java"),
        "class Main {\n \n public static void main(String[] args) {\n \n    } \n}",
    )
    .ok()
    .unwrap();
}

fn makeshell() {
    let shell_execute = if cfg!(target_os = "windows") {
        // } else if cfg!(target_os = "linux") {
        // } else if cfg!(target_os = "macos") {
        // }
    };
}
/*
* contain nix flake that has java installed
* launch a new terminal window with the old one running a bash shell
*
*/

fn check_for_dupe_dir(input_path: &Path) {
    let dirs_read = fs::read_dir("./").unwrap();

    for path in dirs_read {
        if input_path == path.unwrap().path() {
            panic!("Dir name already taken!\n");
        }
    }
}
// make a funcion that checks against current existing directories
// to see if the name of the directory conflicts
