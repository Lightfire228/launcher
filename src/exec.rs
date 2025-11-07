use std::process::Command;
use urlencoding;

use crate::{config::{Project}, terminal};

pub fn launch_project(proj: &Project) {

    fn open_terminal(dir: &str) { terminal::new_window(dir); }

    proj.vscode  .iter().for_each(|x| launch_vscode(x));
    proj.zed     .iter().for_each(|x| launch_zed   (x));
    proj.terminal.iter().for_each(|x| open_terminal(x));

    launch_obsidian(&proj.obsidian);

}

pub fn launch_vscode(path: &str) {
    _open_editor("code", path);
}

pub fn launch_zed(path: &str) {
    _open_editor("zed", path);
}

pub fn launch_obsidian(name: &str) {
    let uri = format!("obsidian://open?vault=Notes&file={}", urlencoding::encode(name));

    _launch_file_uri(&uri);
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
    // use super::*;

    // #[test]
    // fn test_vscode() {
    //     open_vscode("test");
    // }

    // #[test]
    // fn test_obsidian() {
    //     open_obsidian("Game - Silksong");
    // }

}
