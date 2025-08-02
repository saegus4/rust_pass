use std::{env, fs::{self, File}, io::{self, Write}, path::{Path, PathBuf}, process::Command};

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
    let name = &args[3];
    let password_prompt = format!("Enter password for {}:", name);
    let password = rpassword::prompt_password(password_prompt).unwrap();
    let second_password_prompt = format!("Retype password for {}:", name);
    let second_password = rpassword::prompt_password(second_password_prompt).unwrap();

    if !(password == second_password) {
        println!("Error: the entered passwords do not match.");
    }

    let mut temp_password_file = File::create("passwords/temp").expect("Failed to create temp file");
    temp_password_file.write_all(&password.into_bytes()).expect("Failed to write temp file");

    let key = fs::read_to_string("passwords/.gpg-id").expect("Failed to read gpg id file").trim().to_owned();
    let password_file_name = format!("passwords/{}.gpg", name);
    let output = Command::new("gpg")
        .arg("--batch")
        .arg("--yes")
        .arg("--encrypt")
        .arg("--recipient")
        .arg(&key)
        .arg("--output")
        .arg(&password_file_name)
        .arg("passwords/temp")
        .output().unwrap();

    fs::remove_file("passwords/temp").expect("Failed to remove temp file");
}

fn init_pass_vault(args: Vec<String>) {
   let folder_path = &args[3];
    if !Path::new(folder_path).is_dir() {
        fs::create_dir(folder_path).expect("Failed to create folder");
    }

    let gpg_key = Command::new("sh")
        .arg("-c")
        .arg(r#"gpg --list-secret-keys --with-colons --fingerprint \
            | awk -F: '/^fpr:/ {print $10; exit}'"#)
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
