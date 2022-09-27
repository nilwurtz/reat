use std::env;
use std::path::PathBuf;
use std::process::Command;

fn test_dir() -> PathBuf {
    env::current_dir().unwrap()
}

pub fn build() {
    let app_dir = test_dir().parent().unwrap().join("reat");
    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(app_dir)
        .spawn()
        .expect("Failed to build app.")
        .wait()
        .expect("Failed to build app.");
}

pub fn run_app() -> (Command, PathBuf) {
    let app_dir = test_dir()
        .parent()
        .unwrap()
        .join("reat")
        .join("target")
        .join("release");
    (Command::new("./cli"), app_dir)
}
