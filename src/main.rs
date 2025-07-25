use std::{env, fs, path::PathBuf, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    let name = &args[2];

    let file_path: PathBuf = home::home_dir()
        .expect("Home not found")
        .join(".password-store")
        .join(format!("{name}.gpg"));

    match fs::metadata(&file_path) {
        Ok(_) => get_password_value(file_path),
        Err(_) => println!("Tu-tu-ru")
    }
}

fn get_password_value(file_path: PathBuf) {
    let output = Command::new("gpg")
        .arg("--batch")
        .arg("--yes")
        .arg("--decrypt")
        .arg(&file_path)
        .output().unwrap();

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
