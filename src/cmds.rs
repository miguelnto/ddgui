use std::process::{Child, Command};

pub fn dd(dev: &str, iso_file: &str) -> Child {
    let child = Command::new("dd")
        .arg(format!("if={}", iso_file))
        .arg(format!("of={}", dev))
        .arg("status=progress")
        .spawn()
        .unwrap();
    return child;
}
