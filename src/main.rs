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
    make_directory(&project_path.join("src/"), &project_path.join("bin/")); //this is wayy too messy, need to fix later
    makeproject(&project_path.join("src/")); //make Main.java in src/ folder
}
fn make_directory(src_path: &Path, bin_path: &Path) {
    fs::DirBuilder::new()
        .recursive(true)
        .create(src_path)
        .unwrap();
    fs::DirBuilder::new()
        .recursive(true)
        .create(bin_path)
        .unwrap()
}

fn makeproject(path: &Path) {
    fs::write(
        path.join("Main.java"),
        "class Main {\n \n public static void main(String[] args) {\n \n    } \n}",
    )
    .ok()
    .unwrap();
}

<<<<<<< HEAD
// make a funcion that checks against current existing directories
// to see if the name of the directory conflicts
fn check_for_dupe_dir(input_path: &Path) {
    let dirs_read = fs::read_dir("./").unwrap();

    for path in dirs_read {
        if input_path == path.unwrap().path() {
            panic!("Dir name already taken!\n");
        }
    }
}
=======
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
        if input_path == dirs_read. {
            panic!("Dir name already taken!\n");
        }
    }
}
// make a funcion that checks against current existing directories
// to see if the name of the directory conflicts
>>>>>>> dbb9360 (WIP: read_dir comparing to path)
