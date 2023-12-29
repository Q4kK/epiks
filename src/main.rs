use std::fs;
use std::io;
use std::path::*;
use std::Process::Command;

fn main() {
    let path = Path::new("./");
    let mut input = String::new();
    let stdin = io::stdin();
    println!("Hey, you should name the project:");
    stdin.read_line(&mut input).unwrap();
    let project_path = path.join(input.trim());

    make_directory(&project_path).unwrap();
    makeproject(&project_path);

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
            // do this thing Command::new("")
    } else if cfg!(target_os = "linux") {
//do something
    } else if cfg!(target_os = "macos") {
//do something
    }

}
* contain nix flake that has java installed
* launch a new terminal window with the old one running a bash shell
* 
/