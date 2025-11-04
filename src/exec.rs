use std::process::{Command, Output};
use urlencoding;

use crate::config::Project;

pub fn exec() {

    let output = Command::new("sh")
        .arg("-c")
        .arg("echo hello")
        .output()
        .expect("failed to execute process")
    ;

    let hello = str::from_utf8(&output.stdout).expect("Unable to parse output");

    println!(">>> {}", hello);

    open_vscode("test 2");

}

pub fn launch_project(proj: &Project) {

    proj.vscode  .iter().for_each(|x| open_vscode  (x));
    proj.zed     .iter().for_each(|x| open_zed     (x));

    open_obsidian(&proj.obsidian);

    // TODO: terminal
}

pub fn open_vscode(path: &str) {
    _open_editor("code", path);
}

pub fn open_zed(path: &str) {
    _open_editor("zed", path);
}

pub fn open_obsidian(name: &str) {
    let uri = format!("obsidian://open?vault=Notes&file={}", urlencoding::encode(name));

    _launch_file_uri(&uri);
}

pub fn open_terminal(path: &str) {
    Command::new("systemd-run")
        .args(["--user", "konsole", "--workdir", path])
        .output()
        .expect("failed to open konsole")
    ;
}


fn _open_editor(command: &str, path: &str) {
    Command::new(command)
        .arg(path)
        .output()
        .unwrap_or_else(|_| panic!("failed to open {}", command))
    ;
}

fn _launch_file_uri(uri: &str) {
    _open_editor("xdg-open", uri);
}



#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_vscode() {
    //     open_vscode("test");
    // }

    // #[test]
    // fn test_obsidian() {
    //     open_obsidian("Game - Silksong");
    // }

    #[test]
    fn test_konsole() {
        open_terminal("/tmp");
    }

}
