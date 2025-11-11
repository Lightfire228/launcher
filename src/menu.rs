
use std::{collections::HashMap, io::{self, Write}};

use crate::config::{Config, Project};

const VERSION: &str = env!("CARGO_PKG_VERSION");


pub fn display_menu(config: &Config) -> Option<&Project> {

    let choices: HashMap<&str, &Project> = config.projects
        .iter()
        .map(|p| (p.code.as_str(), p))
        .collect()
    ;

    loop {
        print_menu(&config.projects);
    
        let usr_in = prompt();
        let usr_in = usr_in.as_str();

        match get_selection(usr_in, &choices) {
            UserSelection::Quit          => return None,
            UserSelection::Proj(project) => return Some(project),
            
            UserSelection::Invalid       => println!("!! project not found\x07\n"),
        }


    }

}

fn print_menu(projects: &[Project]) {

    println!("Menu:");
    println!(" -- version: {}\n", VERSION);

    for proj in projects {
        println!("  - {:6} - {}", proj.code, proj.name);
    }

    println!("  - {:6} - {}", "q", "Quit");
}

fn get_selection<'a, 'b>(usr_in: &str, choices: &'b HashMap<&str, &'a Project>) -> UserSelection<'a> {
    if usr_in == "q" {
        return UserSelection::Quit;
    }

    let proj = choices.get(usr_in);

    match proj {
        Some(proj) => UserSelection::Proj(*proj),
        None       => UserSelection::Invalid,
    }

}

fn prompt() -> String {

    print!("> ");
    io::stdout().flush().expect("Unable to flush stdout");

    let mut buffer = String::new();
    let     stdin  = io::stdin();

    stdin.read_line(&mut buffer).expect("Unable to read from stdin");

    buffer.trim().to_owned()
}


enum UserSelection<'a> {
    Quit,
    Proj(&'a Project),
    Invalid,
}