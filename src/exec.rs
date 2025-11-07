use std::process::Command;
use urlencoding;

use crate::{config::{Config, Project}, terminal};

pub fn launch_project(config: &Config, proj: &Project) {


    let launch = |iter: &[String], func: Box<dyn Fn(String) -> ()>| { iter
        .iter()
        .map(|x|
            config.expand_dir(x, Some(proj)).unwrap()
        )
        .for_each(|x| func(x));
    };

    fn open_terminal(dir: &str) { terminal::new_window(dir); }

    launch(&proj.vscode,   Box::new(|x| open_vscode  (&x)));
    launch(&proj.zed,      Box::new(|x| open_zed     (&x)));
    launch(&proj.terminal, Box::new(|x| open_terminal(&x)));

    open_obsidian(&proj.obsidian);

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
