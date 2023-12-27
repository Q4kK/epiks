use std::fs;
use std::io;
use std::path::*;

fn main() {
    let path = Path::new("./");
    let mut input = String::new();
    let stdin = io::stdin();
    println!("Hey, you should name the project:");
    stdin.read_line(&mut input).unwrap();
    let project_path = path.join(input);

    make_directory(&project_path).unwrap();
    makeproject(&project_path);

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
}
