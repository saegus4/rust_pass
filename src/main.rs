use std::{env, fs, path::PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();

    let name = &args[2];

    let file_path: PathBuf = home::home_dir()
        .expect("Home not found")
        .join(".password-store")
        .join(format!("{name}.gpg"));

    match fs::metadata(file_path) {
        Ok(_) => println!("Going"),
        Err(_) => println!("Tu-tu-ru")
    }
}
