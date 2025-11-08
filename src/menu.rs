
use std::{collections::HashMap, io::{self, Write}};

use crate::config::{Config, Project};


pub fn display_menu(config: &Config) -> &Project {

    let choices: HashMap<&str, &Project> = config.projects
        .iter()
        .map(|p| (p.code.as_str(), p))
        .collect()
    ;

    println!("Menu:");
    for proj in &config.projects {
        println!("  - {:6} - {}", proj.code, proj.name);
    }

    let usr_in = prompt();
    let proj   = choices.get(usr_in.as_str());

    return proj.expect("Unable to find project");
}


fn prompt() -> String {

    print!("> ");
    io::stdout().flush().expect("Unable to flush stdout");

    let mut buffer = String::new();
    let     stdin  = io::stdin();

    stdin.read_line(&mut buffer).expect("Unable to read from stdin");

    buffer.trim().to_owned()
}
