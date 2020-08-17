use std::process::Command;

fn main() {
    // Compile Gresource
    let out = Command::new("glib-compile-resources")
        .args(&["--generate", "resources.xml"])
        .current_dir("resources")
        .status()
        .expect("Failed to generate resources");
    assert!(out.success());
}
