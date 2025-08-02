use std::{env, fs::{self, File}, io::Write, path::{Path, PathBuf}, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[2];

    match command.as_str() {
        "init" => init_pass_vault(args),
        "insert" => insert_password(args),
        _ => get_password_value(args),
    }
}

fn insert_password(args: Vec<String>) {
}

fn init_pass_vault(args: Vec<String>) {
   let folder_path = &args[3];
    if !Path::new(folder_path).is_dir() {
        fs::create_dir(folder_path).expect("Failed to create folder");
    }

    let gpg_key = Command::new("sh")
        .arg("-c")
        .arg(r#"gpg --list-secret-keys --keyid-format LONG \
            | awk '/^sec/ {split($2,a,"/"); print a[2]}'"#)
        .output()
        .expect("failed to get default gpg key");

    let gpg_key_path = format!("{}{}{}", folder_path, "/", ".gpg-id");
    println!("{}", gpg_key_path);
    let mut gpg_file = File::create(gpg_key_path).expect("Failed to create gpg id file");
    gpg_file.write_all(&gpg_key.stdout).expect("Failed to write gpg id to the gpg file");
}

fn get_password_value(args: Vec<String>) {
    let name = &args[2];

    let file_path: PathBuf = home::home_dir()
        .expect("Home not found")
        .join(".password-store")
        .join(format!("{name}.gpg"));

    let output = Command::new("gpg")
        .arg("--batch")
        .arg("--yes")
        .arg("--decrypt")
        .arg(&file_path)
        .output().unwrap();

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
